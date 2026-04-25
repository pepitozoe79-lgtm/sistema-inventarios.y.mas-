use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::predictivo::PredictiveData;
use crate::models::responses::ApiResponse;
use crate::services::predictive_service::PredictiveService;
use crate::errors::AppError;

/// Obtener proyecciones y alertas predictivas
#[utoipa::path(
    get,
    path = "/api/v1/predictivo",
    responses(
        (status = 200, description = "Proyecciones generadas", body = ApiResponsePredictive),
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_predicciones(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<PredictiveData>>, AppError> {
    let data = PredictiveService::generar_proyecciones(&pool).await?;
    Ok(Json(ApiResponse::new(data)))
}
