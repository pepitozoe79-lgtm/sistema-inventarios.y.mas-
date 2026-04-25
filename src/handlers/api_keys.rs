use axum::{extract::{State, Path}, Json, Extension};
use sqlx::SqlitePool;
use crate::models::api_key::{ApiKey, CrearApiKeyDto, ApiKeyGeneradaResponse};
use crate::models::responses::ApiListResponse;
use crate::errors::AppError;
use crate::auth::Claims;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub async fn listar_keys(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<ApiKey>>, AppError> {
    let keys = sqlx::query_as::<_, ApiKey>("SELECT * FROM api_keys WHERE tenant_id = ?")
        .bind(&claims.tenant_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiListResponse::new(keys)))
}

pub async fn crear_key(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<CrearApiKeyDto>,
) -> Result<Json<ApiKeyGeneradaResponse>, AppError> {
    // Generar llave aleatoria segura
    let raw_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    
    // Hash para la DB
    let mut hasher = Sha256::new();
    hasher.update(&raw_key);
    let hashed_key = hex::encode(hasher.finalize());

    let id = Uuid::new_v4().to_string();
    let scopes = dto.scopes.join(",");

    sqlx::query(
        "INSERT INTO api_keys (id, tenant_id, nombre, hashed_key, scopes) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&claims.tenant_id)
    .bind(&dto.nombre)
    .bind(&hashed_key)
    .bind(&scopes)
    .execute(&pool)
    .await?;

    Ok(Json(ApiKeyGeneradaResponse {
        id,
        key: format!("sk_{}", raw_key), // Prefijo estilo Stripe
    }))
}

pub async fn eliminar_key(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("DELETE FROM api_keys WHERE id = ? AND tenant_id = ?")
        .bind(id)
        .bind(claims.tenant_id)
        .execute(&pool)
        .await?;
    Ok(Json(serde_json::json!({ "mensaje": "API Key eliminada" })))
}
