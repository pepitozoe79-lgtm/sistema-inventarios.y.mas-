use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Venta {
    pub id: String,
    pub usuario_id: String,
    pub total: f64,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DetalleVenta {
    pub id: String,
    pub venta_id: String,
    pub producto_id: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Deserialize)]
pub struct LineaVenta {
    pub producto_id: String,
    pub cantidad: i64,
}

#[derive(Debug, Deserialize)]
pub struct CrearVentaRequest {
    pub lineas: Vec<LineaVenta>,
}

#[derive(Debug, Serialize)]
pub struct VentaCompleta {
    pub venta: Venta,
    pub detalles: Vec<DetalleVenta>,
}

#[derive(serde::Deserialize)]
pub struct FiltrosVentas {
    pub mes: Option<u32>,
    pub anio: Option<i32>,
}
