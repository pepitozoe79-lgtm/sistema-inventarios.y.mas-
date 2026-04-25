use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::superadmin::SuperAdminDashboard;
use crate::models::responses::ApiResponse;
use crate::services::superadmin_service::SuperAdminService;
use crate::errors::AppError;

/// Obtener métricas globales de la plataforma (Sólo SuperAdmin)
#[utoipa::path(
    get,
    path = "/api/v1/superadmin/dashboard",
    responses(
        (status = 200, description = "Dashboard global obtenido", body = ApiResponseSuperAdmin),
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_dashboard_global(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<SuperAdminDashboard>>, AppError> {
    let data = SuperAdminService::obtener_datos_plataforma(&pool).await?;
    Ok(Json(ApiResponse::new(data)))
}
