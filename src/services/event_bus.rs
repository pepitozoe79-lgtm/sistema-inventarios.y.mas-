use sqlx::SqlitePool;
use serde_json::Value;
use crate::services::webhook_service::WebhookService;
use crate::services::integration_service::IntegrationService;
use crate::services::observability_service::ObservabilityService;

pub struct EventBus;

impl EventBus {
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

        // 📊 TELEMETRÍA: Registrar emisión de evento
        let p_metrics = p.clone();
        let tid_metrics = tid.clone();
        tokio::spawn(async move {
            let _ = ObservabilityService::registrar_metrica(&p_metrics, Some(&tid_metrics), "EVENT_BUS", "emit", 1.0).await;
        });

        // 1. Webhooks
        let p1 = p.clone();
        let tid1 = tid.clone();
        let et1 = et.clone();
        let d1 = d.clone();
        tokio::spawn(async move {
            WebhookService::despachar_evento(p1, tid1, et1, d1).await;
        });

        // 2. Marketplace
        let p2 = p.clone();
        let tid2 = tid.clone();
        let et2 = et.clone();
        let d2 = d.clone();
        tokio::spawn(async move {
            IntegrationService::procesar_evento_plataforma(p2, tid2, et2, d2).await;
        });
    }
}
