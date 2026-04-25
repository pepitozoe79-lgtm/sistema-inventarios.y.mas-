use axum::{extract::{State, Path}, Json, Extension};
use sqlx::SqlitePool;
use crate::models::webhook::{WebhookEndpoint, CrearWebhookDto, WebhookLog};
use crate::models::responses::ApiListResponse;
use crate::errors::AppError;
use crate::auth::Claims;
use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub async fn listar_endpoints(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<WebhookEndpoint>>, AppError> {
    let endpoints = sqlx::query_as::<_, WebhookEndpoint>("SELECT * FROM webhook_endpoints WHERE tenant_id = ?")
        .bind(&claims.tenant_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiListResponse::new(endpoints)))
}

pub async fn crear_endpoint(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<CrearWebhookDto>,
) -> Result<Json<WebhookEndpoint>, AppError> {
    let id = format!("wh_{}", Uuid::new_v4());
    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
    
    let event_types = dto.event_types.join(",");

    let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
        "INSERT INTO webhook_endpoints (id, tenant_id, url, secret, event_types) VALUES (?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&claims.tenant_id)
    .bind(&dto.url)
    .bind(format!("whsec_{}", secret))
    .bind(&event_types)
    .fetch_one(&pool)
    .await?;

    Ok(Json(endpoint))
}

pub async fn listar_logs(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<WebhookLog>>, AppError> {
    let logs = sqlx::query_as::<_, WebhookLog>("SELECT * FROM webhook_logs WHERE tenant_id = ? ORDER BY fecha DESC LIMIT 50")
        .bind(&claims.tenant_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiListResponse::new(logs)))
}

pub async fn eliminar_endpoint(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("DELETE FROM webhook_endpoints WHERE id = ? AND tenant_id = ?")
        .bind(id)
        .bind(claims.tenant_id)
        .execute(&pool)
        .await?;
    Ok(Json(serde_json::json!({ "mensaje": "Webhook eliminado" })))
}
