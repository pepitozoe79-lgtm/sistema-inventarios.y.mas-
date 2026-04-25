use sqlx::SqlitePool;
use crate::models::ai::{AiQueryRequest, AiQueryResponse, AiAction};
use crate::services::event_bus::EventBus;
use crate::services::policy_engine::PolicyEngine;
use crate::services::observability_service::ObservabilityService;
use crate::repositories::producto_repository::ProductoRepository;
use crate::repositories::venta_repository::VentaRepository;
use crate::errors::AppError;

pub struct AiService;

impl AiService {
    pub async fn procesar_consulta(
        pool: &SqlitePool,
        tenant_id: &str,
        query: String,
    ) -> Result<AiQueryResponse, AppError> {
        // 📊 TELEMETRÍA: Registrar consulta de IA
        let _ = ObservabilityService::registrar_metrica(pool, Some(tenant_id), "AI", "query", 1.0).await;

        let query_lc = query.to_lowercase();
        let mut actions = Vec::new();
        let mut answer = String::new();

        let action_id = if query_lc.contains("ventas") || query_lc.contains("vendí") {
            "ai.query_sales"
        } else if query_lc.contains("stock") || query_lc.contains("reponer") {
            "ai.query_stock"
        } else if query_lc.contains("alerta") || query_lc.contains("avisa") {
            "ai.trigger_mass_alert"
        } else {
            "ai.unknown_query"
        };

        // GOBERNANZA
        let policy = match PolicyEngine::validar_accion(pool, tenant_id, action_id).await {
            Ok(p) => p,
            Err(e) => {
                let _ = ObservabilityService::registrar_metrica(pool, Some(tenant_id), "AI", "blocked_action", 1.0).await;
                return Ok(AiQueryResponse {
                    answer: format!("Bloqueado por política: {}", e),
                    actions_taken: vec![AiAction { tool: action_id.into(), description: "Policy check".into(), status: "BLOCKED".into() }],
                    suggested_commands: vec!["Mejorar Plan".into()],
                });
            }
        };

        match action_id {
            "ai.query_sales" => {
                let ventas = VentaRepository::listar(pool, tenant_id).await?;
                let total: f64 = ventas.iter().map(|v| v.total).sum();
                answer = format!("Resumen: {} ventas, total ${:.2}.", ventas.len(), total);
                actions.push(AiAction { tool: action_id.into(), description: "Ventas".into(), status: "SUCCESS".into() });
            },
            "ai.query_stock" => {
                let productos = ProductoRepository::listar(pool, tenant_id).await?;
                let criticos: Vec<_> = productos.iter().filter(|p| p.stock_actual <= 5).collect();
                answer = format!("Auditoría: {} productos críticos.", criticos.len());
                actions.push(AiAction { tool: action_id.into(), description: "Stock".into(), status: "SUCCESS".into() });
            },
            "ai.trigger_mass_alert" => {
                EventBus::emitir(pool.clone(), tenant_id.to_string(), "ai.mass_alert", serde_json::json!({ "q": query })).await;
                answer = "Alerta masiva enviada.".into();
                actions.push(AiAction { tool: action_id.into(), description: "Alerta".into(), status: "SUCCESS".into() });
            },
            _ => answer = "No entiendo la petición.".into()
        }

        Ok(AiQueryResponse {
            answer,
            actions_taken: actions,
            suggested_commands: vec!["¿Ventas?".into(), "Stock?".into()],
        })
    }
}
