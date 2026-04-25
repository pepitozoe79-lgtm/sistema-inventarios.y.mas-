use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::dashboard::DashboardData;
use crate::models::responses::ApiResponse;
use crate::services::dashboard_service::DashboardService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn obtener_dashboard(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<DashboardData>>, AppError> {
    let data = DashboardService::obtener_datos(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiResponse::new(data)))
}
