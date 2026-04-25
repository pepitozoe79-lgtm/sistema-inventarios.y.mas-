use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Producto {
    pub id: String,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_unitario: f64,
    pub stock_actual: i64,
    pub creado_en: String,
    pub actualizado_en: String,
}

#[derive(Debug, Deserialize)]
pub struct CrearProductoDto {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_unitario: f64,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarProductoDto {
    pub codigo: Option<String>,
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub precio_unitario: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosProducto {
    pub solo_stock_bajo: Option<bool>,
    pub stock_minimo: Option<i64>,
}
