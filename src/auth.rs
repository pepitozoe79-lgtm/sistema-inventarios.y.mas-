use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sha2::{Sha256, Digest};
use crate::models::api_key::ApiKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // user_id o api_key_id
    pub tenant_id: String,
    pub rol: String,      // "admin", "usuario", "api_key"
    pub scopes: Option<String>, // "products:read,sales:write"
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, tenant_id: &str, rol: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        tenant_id: tenant_id.to_owned(),
        rol: rol.to_owned(),
        scopes: None,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).expect("Token failed")
}

pub async fn auth_middleware(
    State(pool): State<SqlitePool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Intentar Auth por API Key (X-API-Key)
    if let Some(key) = req.headers().get("X-API-Key").and_then(|h| h.to_str().ok()) {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let hashed_key = hex::encode(hasher.finalize());

        let api_key = sqlx::query_as::<_, ApiKey>("SELECT * FROM api_keys WHERE hashed_key = ?")
            .bind(&hashed_key)
            .fetch_optional(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(ak) = api_key {
            let claims = Claims {
                sub: ak.id,
                tenant_id: ak.tenant_id,
                rol: "api_key".into(),
                scopes: Some(ak.scopes),
                exp: 0, // Las API Keys no expiran igual que los JWT
            };
            req.extensions_mut().insert(claims);
            return Ok(next.run(req).await);
        }
    }

    // 2. Intentar Auth por JWT (Authorization)
    let auth_header = req.headers().get(axum::http::header::AUTHORIZATION).and_then(|h| h.to_str().ok());
    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            if let Ok(token_data) = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
                req.extensions_mut().insert(token_data.claims);
                return Ok(next.run(req).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn require_admin(
    axum::Extension(claims): axum::Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if claims.rol == "admin" || claims.rol == "superadmin" {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn require_superadmin(
    axum::Extension(claims): axum::Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if claims.rol == "superadmin" {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
