use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ApiKey {
    pub id: String,
    pub tenant_id: String,
    pub nombre: String,
    pub hashed_key: String,
    pub scopes: String, // "products:read,sales:write"
    pub creado_en: String,
    pub ultima_vez_usada: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearApiKeyDto {
    pub nombre: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyGeneradaResponse {
    pub id: String,
    pub key: String, // La llave en texto plano (solo se muestra una vez)
}

impl ApiKey {
    pub fn tiene_scope(&self, scope: &str) -> bool {
        self.scopes.split(',').any(|s| s.trim() == scope)
    }
}
