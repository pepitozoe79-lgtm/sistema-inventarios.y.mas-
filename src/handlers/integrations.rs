use axum::{extract::{State, Path}, Json, Extension};
use sqlx::SqlitePool;
use crate::models::integration::{IntegrationAppFull, InstalarAppDto};
use crate::services::integration_service::IntegrationService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn listar_marketplace(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<IntegrationAppFull>>, AppError> {
    let apps = IntegrationService::listar_marketplace(&pool, &claims.tenant_id).await?;
    Ok(Json(apps))
}

pub async fn instalar_app(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<InstalarAppDto>,
) -> Result<Json<serde_json::Value>, AppError> {
    IntegrationService::instalar_app(&pool, &claims.tenant_id, &dto.app_id, dto.config_json).await?;
    Ok(Json(serde_json::json!({ "mensaje": "Aplicación instalada con éxito" })))
}

pub async fn desinstalar_app(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(app_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("DELETE FROM tenant_integrations WHERE tenant_id = ? AND app_id = ?")
        .bind(&claims.tenant_id)
        .bind(app_id)
        .execute(&pool)
        .await?;
    Ok(Json(serde_json::json!({ "mensaje": "Aplicación desinstalada" })))
}
