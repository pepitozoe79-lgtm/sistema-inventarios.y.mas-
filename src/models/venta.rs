use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Venta {
    pub id: String,
    pub usuario_id: String,
    pub total: f64,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DetalleVenta {
    pub id: String,
    pub venta_id: String,
    pub producto_id: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LineaVentaDto {
    pub producto_id: String,
    pub cantidad: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearVentaDto {
    pub lineas: Vec<LineaVentaDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VentaCompletaResponse {
    pub venta: Venta,
    pub detalles: Vec<DetalleVenta>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseVenta {
    pub data: VentaCompletaResponse,
}

#[derive(Serialize, ToSchema)]
pub struct ApiListResponseVenta {
    pub data: Vec<Venta>,
    pub meta: crate::models::responses::Meta,
}
