use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::analytics::AnalyticsData;
use crate::models::responses::ApiResponse;
use crate::services::analytics_service::AnalyticsService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn obtener_analytics(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<AnalyticsData>>, AppError> {
    let data = AnalyticsService::obtener_analytics(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiResponse::new(data)))
}
