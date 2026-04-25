use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::predictivo::PredictiveData;
use crate::models::responses::ApiResponse;
use crate::services::predictive_service::PredictiveService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn obtener_predicciones(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<PredictiveData>>, AppError> {
    let data = PredictiveService::generar_proyecciones(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiResponse::new(data)))
}
