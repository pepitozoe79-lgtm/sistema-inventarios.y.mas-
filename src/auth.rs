use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::usuario::{AuthResponse, LoginRequest, NuevoUsuario, Usuario, UsuarioPublico};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub rol: String,
    pub exp: usize,
}

const JWT_EXPIRATION_HOURS: usize = 24;

pub async fn registro(
    State(pool): State<SqlitePool>,
    Json(nuevo): Json<NuevoUsuario>,
) -> Result<Json<AuthResponse>, (axum::http::StatusCode, String)> {
    if nuevo.username.trim().is_empty() || nuevo.password.trim().is_empty() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Usuario y contraseña requeridos".into()));
    }

    let id = Uuid::new_v4().to_string();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(nuevo.password.as_bytes(), &salt)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    sqlx::query(
        "INSERT INTO usuarios (id, username, password_hash, rol) VALUES (?, ?, ?, 'usuario')"
    )
    .bind(&id)
    .bind(&nuevo.username)
    .bind(&password_hash)
    .execute(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::CONFLICT, format!("Error al crear usuario: {}", e)))?;

    let token = generar_token(&id, &nuevo.username, "usuario")?;
    let usuario_publico = UsuarioPublico {
        id,
        username: nuevo.username,
        rol: "usuario".into(),
    };

    Ok(Json(AuthResponse { token, usuario: usuario_publico }))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(login): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (axum::http::StatusCode, String)> {
    let usuario = sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios WHERE username = ?")
        .bind(&login.username)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Credenciales inválidas".into()))?;

    let parsed_hash = PasswordHash::new(&usuario.password_hash)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Argon2::default()
        .verify_password(login.password.as_bytes(), &parsed_hash)
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Credenciales inválidas".into()))?;

    let token = generar_token(&usuario.id, &usuario.username, &usuario.rol)?;
    let usuario_publico = UsuarioPublico {
        id: usuario.id,
        username: usuario.username,
        rol: usuario.rol,
    };
    Ok(Json(AuthResponse { token, usuario: usuario_publico }))
}

fn generar_token(user_id: &str, username: &str, rol: &str) -> Result<String, (axum::http::StatusCode, String)> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "supersecreto".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(JWT_EXPIRATION_HOURS as i64))
        .expect("timestamp inválido")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        username: username.to_owned(),
        rol: rol.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn auth_middleware(
    State(_pool): State<SqlitePool>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let auth_header = req.headers().get("Authorization").and_then(|v| v.to_str().ok());
    if let Some(token) = auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "supersecreto".to_string());
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
        req.extensions_mut().insert(token_data.claims);
        Ok(next.run(req).await)
    } else {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}

pub async fn require_admin(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let claims = req.extensions().get::<Claims>().ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    if claims.rol != "admin" {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
