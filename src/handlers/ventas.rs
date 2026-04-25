use axum::{
    extract::State,
    Json, Extension
};
use sqlx::SqlitePool;
use crate::models::venta::{Venta, CrearVentaDto, VentaCompletaResponse};
use crate::models::responses::{ApiResponse, ApiListResponse};
use crate::services::venta_service::VentaService;
use crate::errors::AppError;
use crate::auth::Claims;

/// Listar historial de ventas
#[utoipa::path(
    get,
    path = "/api/v1/ventas",
    responses(
        (status = 200, description = "Lista de ventas obtenida", body = ApiListResponseVenta),
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_ventas(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiListResponse<Venta>>, AppError> {
    let ventas = VentaService::listar_ventas(&pool).await?;
    Ok(Json(ApiListResponse::new(ventas)))
}

/// Crear una nueva venta (Transaccional)
#[utoipa::path(
    post,
    path = "/api/v1/ventas",
    request_body = CrearVentaDto,
    responses(
        (status = 201, description = "Venta realizada con éxito", body = ApiResponseVenta),
        (status = 409, description = "Stock insuficiente para uno o más productos"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_venta(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<CrearVentaDto>,
) -> Result<Json<ApiResponse<VentaCompletaResponse>>, AppError> {
    let venta_completa = VentaService::crear_venta(&pool, &claims.sub, dto).await?;
    Ok(Json(ApiResponse::new(venta_completa)))
}
