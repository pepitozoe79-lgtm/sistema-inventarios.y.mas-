use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::predictivo::PredictiveData;
use crate::models::responses::ApiResponse;
use crate::services::predictive_service::PredictiveService;
use crate::services::plan_service::PlanService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn obtener_predicciones(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PredictiveData>>, AppError> {
    // 🔒 SaaS Enforcement: Verificar acceso a Predicciones
    PlanService::validar_acceso_feature(&pool, &claims.tenant_id, "PREDICTIVO").await?;

    let data = PredictiveService::generar_proyecciones(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiResponse::new(data)))
}
