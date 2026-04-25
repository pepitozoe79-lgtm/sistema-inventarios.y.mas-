use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Gasto {
    pub id: String,
    pub tipo: String, // "OPERATIVO", "MERCADERIA", "SUELDOS", "OTROS"
    pub monto: f64,
    pub descripcion: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearGastoDto {
    pub tipo: String,
    pub monto: f64,
    pub descripcion: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseGasto {
    pub data: Gasto,
}

#[derive(Serialize, ToSchema)]
pub struct ApiListResponseGasto {
    pub data: Vec<Gasto>,
    pub meta: crate::models::responses::Meta,
}
