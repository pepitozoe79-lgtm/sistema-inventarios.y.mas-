use axum::{extract::State, Json};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::models::responses::ApiResponse;
use crate::auth::create_jwt;
use crate::errors::AppError;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegistroSaaSRequest {
    pub nombre_empresa: String,
    pub username_admin: String,
    pub password_admin: String,
}

#[derive(Serialize)]
pub struct RegistroSaaSResponse {
    pub tenant_id: String,
    pub admin_id: String,
}

pub async fn registro_saas(
    State(pool): State<SqlitePool>,
    Json(dto): Json<RegistroSaaSRequest>,
) -> Result<Json<ApiResponse<RegistroSaaSResponse>>, AppError> {
    let mut tx = pool.begin().await?;

    // 1. Crear el Tenant
    let tenant_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO tenants (id, nombre, plan) VALUES (?, ?, 'BASIC')")
        .bind(&tenant_id)
        .bind(&dto.nombre_empresa)
        .execute(&mut *tx)
        .await?;

    // 2. Crear el Usuario Administrador para ese Tenant
    let admin_id = Uuid::new_v4().to_string();
    let password_hash = bcrypt::hash(dto.password_admin, bcrypt::DEFAULT_COST).unwrap();
    
    sqlx::query("INSERT INTO usuarios (id, tenant_id, username, password_hash, rol) VALUES (?, ?, ?, ?, 'admin')")
        .bind(&admin_id)
        .bind(&tenant_id)
        .bind(&dto.username_admin)
        .bind(&password_hash)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(ApiResponse::new(RegistroSaaSResponse { tenant_id, admin_id })))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: crate::models::usuario::Usuario,
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(dto): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let user = sqlx::query_as::<_, crate::models::usuario::Usuario>(
        "SELECT * FROM usuarios WHERE username = ?"
    )
    .bind(&dto.username)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Credenciales inválidas".into()))?;

    if bcrypt::verify(dto.password, &user.password_hash).unwrap() {
        let token = create_jwt(&user.id, &user.tenant_id, &user.rol);
        Ok(Json(ApiResponse::new(LoginResponse { token, usuario: user })))
    } else {
        Err(AppError::Unauthorized("Credenciales inválidas".into()))
    }
}
