use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::ai::{AiQueryRequest, AiQueryResponse};
use crate::services::ai_service::AiService;
use crate::errors::AppError;
use crate::auth::Claims;

/// [v2] Consulta al Asistente de IA (Cognitive Gateway)
#[utoipa::path(
    post,
    path = "/api/v2/ai/query",
    responses(
        (status = 200, description = "Respuesta del Asistente IA", body = AiQueryResponse),
    ),
    security(("bearer_auth" = []), ("api_key" = []))
)]
pub async fn query_ai(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<AiQueryRequest>,
) -> Result<Json<AiQueryResponse>, AppError> {
    let response = AiService::procesar_consulta(&pool, &claims.tenant_id, dto.query).await?;
    Ok(Json(response))
}
