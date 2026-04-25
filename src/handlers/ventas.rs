use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::venta::{Venta, CrearVentaDto, VentaCompletaResponse};
use crate::models::responses::ApiListResponse;
use crate::services::venta_service::VentaService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn listar_ventas(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<Venta>>, AppError> {
    let ventas = VentaService::listar_ventas(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiListResponse::new(ventas)))
}

pub async fn crear_venta(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<CrearVentaDto>,
) -> Result<Json<VentaCompletaResponse>, AppError> {
    let respuesta = VentaService::crear_venta(&pool, &claims.tenant_id, &claims.sub, dto).await?;
    Ok(Json(respuesta))
}
