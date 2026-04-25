use sqlx::SqlitePool;
use crate::models::ai::{AiQueryRequest, AiQueryResponse, AiAction};
use crate::services::event_bus::EventBus;
use crate::services::policy_engine::PolicyEngine;
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
        let query_lc = query.to_lowercase();
        let mut actions = Vec::new();
        let mut answer = String::new();
        let mut suggestions = Vec::new();

        // --- 1. Mapear Intención a Acción Interna ---
        let action_id = if query_lc.contains("ventas") || query_lc.contains("vendí") {
            "ai.query_sales"
        } else if query_lc.contains("stock") || query_lc.contains("reponer") {
            "ai.query_stock"
        } else if query_lc.contains("alerta") || query_lc.contains("avisa") {
            "ai.trigger_mass_alert"
        } else {
            "ai.unknown_query"
        };

        // --- 2. GOBERNANZA: Validar contra el Policy Engine ---
        let policy = match PolicyEngine::validar_accion(pool, tenant_id, action_id).await {
            Ok(p) => p,
            Err(e) => {
                return Ok(AiQueryResponse {
                    answer: format!("Bloqueado por política de seguridad: {}", e),
                    actions_taken: vec![AiAction {
                        tool: action_id.into(),
                        description: "Validación de gobernanza".into(),
                        status: "BLOCKED".into(),
                    }],
                    suggested_commands: vec!["Mejorar a Plan PRO".into()],
                });
            }
        };

        // --- 3. EJECUCIÓN (Si el Policy Engine lo permitió) ---
        match action_id {
            "ai.query_sales" => {
                let ventas = VentaRepository::listar(pool, tenant_id).await?;
                let total: f64 = ventas.iter().map(|v| v.total).sum();
                answer = format!("Confirmado. Se han procesado {} ventas hoy por un total de ${:.2}.", ventas.len(), total);
                actions.push(AiAction { tool: action_id.into(), description: "Consulta financiera autorizada".into(), status: "SUCCESS".into() });
            },
            "ai.query_stock" => {
                let productos = ProductoRepository::listar(pool, tenant_id).await?;
                let criticos: Vec<_> = productos.iter().filter(|p| p.stock_actual <= 5).collect();
                answer = if criticos.is_empty() { "Inventario saludable. No hay alertas críticas.".into() } 
                         else { format!("He auditado el stock: hay {} productos en riesgo.", criticos.len()) };
                actions.push(AiAction { tool: action_id.into(), description: "Auditoría de inventario autorizada".into(), status: "SUCCESS".into() });
            },
            "ai.trigger_mass_alert" => {
                EventBus::emitir(pool.clone(), tenant_id.to_string(), "ai.mass_alert", serde_json::json!({ "query": query })).await;
                answer = "Alerta masiva disparada con éxito. El EventBus ha notificado a todas las integraciones.".into();
                actions.push(AiAction { tool: action_id.into(), description: "Emisión de evento masivo autorizada".into(), status: "SUCCESS".into() });
            },
            _ => {
                answer = "No entiendo esa instrucción o no tengo permiso para ejecutarla.".into();
            }
        }

        Ok(AiQueryResponse {
            answer,
            actions_taken: actions,
            suggested_commands: vec!["¿Cuánto vendí hoy?".into(), "Estado del stock".into()],
        })
    }
}
