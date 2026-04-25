use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct DashboardStats {
    pub ventas_hoy_total: f64,
    pub ventas_hoy_cantidad: i64,
    pub productos_vendidos_hoy: i64,
    pub alertas_stock_bajo: i64,
    pub gastos_hoy: f64,
    pub utilidad_hoy: f64,
}

#[derive(Serialize, ToSchema)]
pub struct TopProducto {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Serialize, ToSchema)]
pub struct ActividadReciente {
    pub tipo: String, // "VENTA" o "MOVIMIENTO"
    pub descripcion: String,
    pub fecha: String,
}

#[derive(Serialize, ToSchema)]
pub struct DashboardData {
    pub stats: DashboardStats,
    pub top_productos: Vec<TopProducto>,
    pub actividad: Vec<ActividadReciente>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseDashboard {
    pub data: DashboardData,
}
