use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Tenant {
    pub id: String,
    pub nombre: String,
    pub plan: String, // "BASIC", "PRO", "ENTERPRISE"
    pub creado_en: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearTenantDto {
    pub nombre: String,
}
