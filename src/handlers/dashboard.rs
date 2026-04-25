use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::dashboard::DashboardData;
use crate::models::responses::ApiResponse;
use crate::services::dashboard_service::DashboardService;
use crate::errors::AppError;

/// Obtener datos consolidados para el dashboard
#[utoipa::path(
    get,
    path = "/api/v1/dashboard",
    responses(
        (status = 200, description = "Datos del dashboard obtenidos", body = ApiResponseDashboard),
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_dashboard(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<DashboardData>>, AppError> {
    let data = DashboardService::obtener_datos(&pool).await?;
    Ok(Json(ApiResponse::new(data)))
}
