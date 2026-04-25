use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct IntegrationApp {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub logo_url: Option<String>,
    pub eventos_requeridos: String,
    pub config_schema: Option<String>,
    pub premium: bool,
    pub creado_en: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TenantIntegration {
    pub id: String,
    pub tenant_id: String,
    pub app_id: String,
    pub config_json: String,
    pub status: String,
    pub creado_en: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InstalarAppDto {
    pub app_id: String,
    pub config_json: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IntegrationAppFull {
    pub app: IntegrationApp,
    pub instalada: bool,
    pub status: Option<String>,
}
