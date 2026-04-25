use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::analytics::AnalyticsData;
use crate::models::responses::ApiResponse;
use crate::services::analytics_service::AnalyticsService;
use crate::errors::AppError;

/// Obtener analítica avanzada e inteligencia de negocio
#[utoipa::path(
    get,
    path = "/api/v1/analytics",
    responses(
        (status = 200, description = "Analítica obtenida", body = ApiResponseAnalytics),
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_analytics(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<AnalyticsData>>, AppError> {
    let data = AnalyticsService::obtener_analytics(&pool).await?;
    Ok(Json(ApiResponse::new(data)))
}
