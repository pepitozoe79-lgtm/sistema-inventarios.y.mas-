use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct WebhookEndpoint {
    pub id: String,
    pub tenant_id: String,
    pub url: String,
    pub secret: String,
    pub event_types: String, // "sale.created,stock.low"
    pub active: bool,
    pub creado_en: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearWebhookDto {
    pub url: String,
    pub event_types: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebhookEventPayload {
    pub event_id: String,
    pub event_type: String,
    pub tenant_id: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct WebhookLog {
    pub id: String,
    pub endpoint_id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub status_code: Option<i32>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub intentos: i32,
    pub fecha: String,
}
