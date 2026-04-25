use sqlx::SqlitePool;
use serde_json::Value;
use uuid::Uuid;
use crate::models::integration::{IntegrationApp, TenantIntegration};
use crate::services::webhook_service::WebhookService;
use crate::errors::AppError;

pub struct IntegrationService;

impl IntegrationService {
    /// Despacha un evento interno al sistema de integraciones (Marketplace)
    pub async fn procesar_evento_plataforma(
        pool: SqlitePool,
        tenant_id: String,
        event_type: String,
        data: Value,
    ) {
        // 1. Buscar integraciones instaladas por el tenant que escuchen este evento
        let instalaciones = match sqlx::query_as::<_, TenantIntegration>(
            r#"
            SELECT ti.* FROM tenant_integrations ti
            JOIN integration_apps ia ON ti.app_id = ia.id
            WHERE ti.tenant_id = ? AND ti.status = 'ACTIVE' AND ia.eventos_requeridos LIKE ?
            "#
        )
        .bind(&tenant_id)
        .bind(format!("%{}%", event_type))
        .fetch_all(&pool)
        .await {
            Ok(inst) => inst,
            Err(_) => return,
        };

        for instalacion in instalaciones {
            let pool_clone = pool.clone();
            let app_id = instalacion.app_id.clone();
            let config = instalacion.config_json.clone();
            let tenant_id_clone = tenant_id.clone();
            let event_type_clone = event_type.clone();
            let data_clone = data.clone();

            // Ejecución asíncrona de la lógica de la App
            tokio::spawn(async move {
                Self::ejecutar_logica_app(
                    pool_clone,
                    app_id,
                    tenant_id_clone,
                    event_type_clone,
                    data_clone,
                    config
                ).await;
            });
        }
    }

    async fn ejecutar_logica_app(
        _pool: SqlitePool,
        app_id: String,
        tenant_id: String,
        event_type: String,
        data: Value,
        _config: String,
    ) {
        // NOTA: En un sistema real, aquí invocarías el "Worker" o "Lambda" de la App.
        // Simulamos la ejecución según el tipo de aplicación:
        
        println!("🚀 Marketplace Engine: Ejecutando App [{}] para Tenant [{}] ante evento [{}]", app_id, tenant_id, event_type);

        match app_id.as_str() {
            "shopify_sync" => {
                // Simulación: Llamada a Shopify API para sincronizar venta
                println!("📦 Shopify: Sincronizando datos de {}...", event_type);
            },
            "whatsapp_notify" => {
                // Simulación: Envío de mensaje vía WhatsApp Business API
                println!("📱 WhatsApp: Enviando alerta de {}...", event_type);
            },
            "google_sheets" => {
                // Simulación: Inserción de fila en Sheets
                println!("📊 Google Sheets: Exportando datos de {}...", event_type);
            },
            _ => {}
        }
    }

    pub async fn listar_marketplace(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<crate::models::integration::IntegrationAppFull>, AppError> {
        let apps = sqlx::query_as::<_, IntegrationApp>("SELECT * FROM integration_apps")
            .fetch_all(pool)
            .await?;

        let instalaciones = sqlx::query_as::<_, TenantIntegration>("SELECT * FROM tenant_integrations WHERE tenant_id = ?")
            .bind(tenant_id)
            .fetch_all(pool)
            .await?;

        let mut resultado = Vec::new();
        for app in apps {
            let instalacion = instalaciones.iter().find(|i| i.app_id == app.id);
            resultado.push(crate::models::integration::IntegrationAppFull {
                app,
                instalada: instalacion.is_some(),
                status: instalacion.map(|i| i.status.clone()),
            });
        }
        Ok(resultado)
    }

    pub async fn instalar_app(pool: &SqlitePool, tenant_id: &str, app_id: &str, config: Value) -> Result<(), AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO tenant_integrations (id, tenant_id, app_id, config_json) VALUES (?, ?, ?, ?)"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(app_id)
        .bind(config.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }
}
