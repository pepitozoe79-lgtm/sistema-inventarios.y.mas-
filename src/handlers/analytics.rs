use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::analytics::AnalyticsData;
use crate::models::responses::ApiResponse;
use crate::services::analytics_service::AnalyticsService;
use crate::services::plan_service::PlanService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn obtener_analytics(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<AnalyticsData>>, AppError> {
    // 🔒 SaaS Enforcement: Verificar acceso a BI
    PlanService::validar_acceso_feature(&pool, &claims.tenant_id, "BI").await?;

    let data = AnalyticsService::obtener_analytics(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiResponse::new(data)))
}
