use axum::{
    extract::{State, Query},
    Json, Extension
};
use sqlx::SqlitePool;
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto, FiltrosMovimiento};
use crate::models::responses::{ApiResponse, ApiListResponse};
use crate::services::movimiento_service::MovimientoService;
use crate::errors::{AppError, ErrorResponse};
use crate::auth::Claims;
use crate::models::producto::ApiResponseProducto; // Para reusar si fuera necesario

/// Listar historial de movimientos (Kardex)
#[utoipa::path(
    get,
    path = "/api/v1/inventario/movimientos",
    responses(
        (status = 200, description = "Historial obtenido", body = ApiListResponseMovimiento),
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_movimientos(
    State(pool): State<SqlitePool>,
    Query(_filtros): Query<FiltrosMovimiento>,
) -> Result<Json<ApiListResponse<MovimientoInventario>>, AppError> {
    let movimientos = MovimientoService::listar_movimientos(&pool).await?;
    Ok(Json(ApiListResponse::new(movimientos)))
}

/// Registrar un nuevo movimiento (Entrada/Salida/Ajuste)
#[utoipa::path(
    post,
    path = "/api/v1/inventario/movimientos",
    request_body = NuevoMovimientoDto,
    responses(
        (status = 201, description = "Movimiento registrado", body = ApiResponseMovimiento),
        (status = 409, description = "Stock insuficiente", body = ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn registrar(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<NuevoMovimientoDto>,
) -> Result<Json<ApiResponse<MovimientoInventario>>, AppError> {
    let reg = MovimientoService::registrar_movimiento(&pool, dto, Some(claims.sub)).await?;
    Ok(Json(ApiResponse::new(reg)))
}
