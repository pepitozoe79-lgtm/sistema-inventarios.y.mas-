use axum::{extract::{State, Path}, Json};
use sqlx::SqlitePool;
use crate::models::gasto::{Gasto, CrearGastoDto};
use crate::models::responses::{ApiResponse, ApiListResponse};
use crate::repositories::gasto_repository::GastoRepository;
use crate::errors::AppError;

/// Listar todos los gastos
#[utoipa::path(
    get,
    path = "/api/v1/gastos",
    responses(
        (status = 200, description = "Lista de gastos obtenida", body = ApiListResponseGasto),
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiListResponse<Gasto>>, AppError> {
    let gastos = GastoRepository::listar(&pool).await?;
    Ok(Json(ApiListResponse::new(gastos)))
}

/// Registrar un nuevo gasto
#[utoipa::path(
    post,
    path = "/api/v1/gastos",
    request_body = CrearGastoDto,
    responses(
        (status = 201, description = "Gasto registrado con éxito", body = ApiResponseGasto),
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear(
    State(pool): State<SqlitePool>,
    Json(dto): Json<CrearGastoDto>,
) -> Result<Json<ApiResponse<Gasto>>, AppError> {
    let gasto = GastoRepository::crear(&pool, dto).await?;
    Ok(Json(ApiResponse::new(gasto)))
}

/// Eliminar un gasto
#[utoipa::path(
    delete,
    path = "/api/v1/gastos/{id}",
    params(("id" = String, Path, description = "ID del gasto")),
    responses(
        (status = 200, description = "Gasto eliminado"),
        (status = 404, description = "Gasto no encontrado"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    GastoRepository::eliminar(&pool, &id).await?;
    Ok(Json(serde_json::json!({"mensaje": "Gasto eliminado"})))
}
