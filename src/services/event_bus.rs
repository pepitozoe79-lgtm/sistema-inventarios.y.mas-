use sqlx::SqlitePool;
use serde_json::Value;
use crate::services::webhook_service::WebhookService;
use crate::services::integration_service::IntegrationService;

pub struct EventBus;

impl EventBus {
    /// El punto único de emisión de eventos de la plataforma.
    /// Se encarga de notificar a Webhooks externos y de ejecutar Apps del Marketplace.
    pub async fn emitir(
        pool: SqlitePool,
        tenant_id: String,
        event_type: &str,
        data: Value,
    ) {
        let et = event_type.to_string();
        let tid = tenant_id.clone();
        let p = pool.clone();
        let d = data.clone();

        // 1. Notificar a Webhooks Outbound
        let p1 = p.clone();
        let tid1 = tid.clone();
        let et1 = et.clone();
        let d1 = d.clone();
        tokio::spawn(async move {
            WebhookService::despachar_evento(p1, tid1, et1, d1).await;
        });

        // 2. Ejecutar Apps del Marketplace
        let p2 = p.clone();
        let tid2 = tid.clone();
        let et2 = et.clone();
        let d2 = d.clone();
        tokio::spawn(async move {
            IntegrationService::procesar_evento_plataforma(p2, tid2, et2, d2).await;
        });

        println!("📡 EventBus: Evento [{}] emitido para Tenant [{}]", event_type, tenant_id);
    }
}
