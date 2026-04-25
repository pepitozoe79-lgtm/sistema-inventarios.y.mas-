use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PlatformMetric {
    pub id: i32,
    pub tenant_id: Option<String>,
    pub categoria: String,
    pub metrica: String,
    pub valor: f64,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct SystemHealth {
    pub id: String,
    pub componente: String,
    pub status: String,
    pub latencia_ms: Option<i32>,
    pub actualizado_en: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalDashboardMetrics {
    pub mrr_total: f64,
    pub active_tenants: i64,
    pub ai_calls_total: i64,
    pub events_total: i64,
    pub webhook_success_rate: f64,
    pub health_status: Vec<SystemHealth>,
}
