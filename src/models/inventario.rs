use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct MovimientoInventario {
    pub id: String,
    pub producto_id: String,
    pub tipo: String,        // "ENTRADA", "SALIDA", "AJUSTE"
    pub cantidad: i64,
    pub stock_antes: i64,
    pub stock_despues: i64,
    pub costo_unitario: Option<f64>,
    pub motivo: Option<String>,
    pub usuario_id: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NuevoMovimientoDto {
    pub producto_id: String,
    pub tipo: String,        // "ENTRADA", "SALIDA", "AJUSTE"
    pub cantidad: i64,
    pub costo_unitario: Option<f64>,
    pub motivo: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FiltrosMovimiento {
    pub producto_id: Option<String>,
    pub tipo: Option<String>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseMovimiento {
    pub data: MovimientoInventario,
}

#[derive(Serialize, ToSchema)]
pub struct ApiListResponseMovimiento {
    pub data: Vec<MovimientoInventario>,
    pub meta: crate::models::responses::Meta,
}
