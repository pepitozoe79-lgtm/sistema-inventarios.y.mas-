use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MovimientoInventario {
    pub id: String,
    pub producto_id: String,
    pub tipo: String,
    pub cantidad: i64,
    pub motivo: Option<String>,
    pub usuario_id: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoMovimiento {
    pub producto_id: String,
    pub cantidad: i64,
    pub motivo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosMovimiento {
    pub producto_id: Option<String>,
    pub tipo: Option<String>,        // "entrada" o "salida"
    pub fecha_desde: Option<String>, // formato YYYY-MM-DD
    pub fecha_hasta: Option<String>,
}
