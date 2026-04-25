use sqlx::SqlitePool;
use serde_json::Value;
use uuid::Uuid;
use crate::models::webhook::{WebhookEndpoint, WebhookEventPayload};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use reqwest::Client;
use std::time::Duration;

pub struct WebhookService;

impl WebhookService {
    /// Despacha un evento a todos los endpoints suscritos de un tenant
    pub async fn despachar_evento(
        pool: SqlitePool,
        tenant_id: String,
        event_type: String,
        data: Value,
    ) {
        // 1. Buscar endpoints activos para este tenant y tipo de evento
        let endpoints = match sqlx::query_as::<_, WebhookEndpoint>(
            "SELECT * FROM webhook_endpoints WHERE tenant_id = ? AND active = 1 AND event_types LIKE ?"
        )
        .bind(&tenant_id)
        .bind(format!("%{}%", event_type))
        .fetch_all(&pool)
        .await {
            Ok(e) => e,
            Err(_) => return,
        };

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        for endpoint in endpoints {
            let pool_clone = pool.clone();
            let endpoint_clone = endpoint.clone();
            let event_type_clone = event_type.clone();
            let data_clone = data.clone();
            let tenant_id_clone = tenant_id.clone();
            let client_clone = client.clone();

            // Despacho asíncrono para no bloquear el hilo principal
            tokio::spawn(async move {
                Self::enviar_a_endpoint(
                    pool_clone,
                    client_clone,
                    endpoint_clone,
                    tenant_id_clone,
                    event_type_clone,
                    data_clone
                ).await;
            });
        }
    }

    async fn enviar_a_endpoint(
        pool: SqlitePool,
        client: Client,
        endpoint: WebhookEndpoint,
        tenant_id: String,
        event_type: String,
        data: Value,
    ) {
        let event_id = format!("evt_{}", Uuid::new_v4());
        let payload = WebhookEventPayload {
            event_id: event_id.clone(),
            event_type: event_type.clone(),
            tenant_id: tenant_id.clone(),
            data: data.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let body = serde_json::to_string(&payload).unwrap();

        // 2. Firmar el payload con HMAC SHA-256
        let mut mac = Hmac::<Sha256>::new_from_slice(endpoint.secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // 3. Enviar Request
        let response = client.post(&endpoint.url)
            .header("X-Stripe-Signature", format!("sha256={}", signature)) // Estilo Stripe
            .header("X-Event-Type", &event_type)
            .header("Content-Type", "application/json")
            .body(body.clone())
            .send()
            .await;

        // 4. Registrar Log
        let (status_code, response_body) = match response {
            Ok(resp) => (Some(resp.status().as_u16() as i32), resp.text().await.unwrap_or_default()),
            Err(e) => (None, e.to_string()),
        };

        let log_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO webhook_logs (id, endpoint_id, tenant_id, event_type, status_code, request_body, response_body) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log_id)
        .bind(endpoint.id)
        .bind(tenant_id)
        .bind(event_type)
        .bind(status_code)
        .bind(body)
        .bind(response_body)
        .execute(&pool)
        .await;
    }
}
