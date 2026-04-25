use axum::{extract::{State, Query}, Json, Extension};
use sqlx::SqlitePool;
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use crate::models::responses::ApiListResponse;
use crate::services::movimiento_service::MovimientoService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn listar_movimientos(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<MovimientoInventario>>, AppError> {
    let movimientos = MovimientoService::listar_movimientos(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiListResponse::new(movimientos)))
}

pub async fn registrar(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<NuevoMovimientoDto>,
) -> Result<Json<MovimientoInventario>, AppError> {
    let movimiento = MovimientoService::registrar_movimiento(&pool, &claims.tenant_id, claims.sub, dto).await?;
    Ok(Json(movimiento))
}
