use sqlx::SqlitePool;
use crate::models::ai::{AiQueryRequest, AiQueryResponse, AiAction};
use crate::services::event_bus::EventBus;
use crate::repositories::producto_repository::ProductoRepository;
use crate::repositories::venta_repository::VentaRepository;
use crate::errors::AppError;

pub struct AiService;

impl AiService {
    /// Procesa una consulta en lenguaje natural y decide qué herramientas (tools) ejecutar.
    pub async fn procesar_consulta(
        pool: &SqlitePool,
        tenant_id: &str,
        query: String,
    ) -> Result<AiQueryResponse, AppError> {
        let query_lc = query.to_lowercase();
        let mut actions = Vec::new();
        let mut answer = String::new();
        let mut suggestions = Vec::new();

        // 🧠 Motor de Inferencia Simple (Expandible a LLM)
        if query_lc.contains("ventas") || query_lc.contains("vendí") || query_lc.contains("dinero") {
            // Tool: Resumen de Ventas
            let ventas = VentaRepository::listar(pool, tenant_id).await?;
            let total: f64 = ventas.iter().map(|v| v.total).sum();
            answer = format!("Has realizado {} ventas hoy, con un total de ${:.2}.", ventas.len(), total);
            actions.push(AiAction {
                tool: "VentaRepository::listar".into(),
                description: "Consulta de transacciones del día".into(),
                status: "SUCCESS".into(),
            });
            suggestions.push("Ver detalle de ventas".into());
        } 
        else if query_lc.contains("stock") || query_lc.contains("reponer") || query_lc.contains("quedan") {
            // Tool: Análisis de Stock Crítico
            let productos = ProductoRepository::listar(pool, tenant_id).await?;
            let criticos: Vec<_> = productos.iter().filter(|p| p.stock_actual <= 5).collect();
            
            if criticos.is_empty() {
                answer = "Todo está bajo control. No tienes productos con stock crítico en este momento.".into();
            } else {
                answer = format!("Atención: Tienes {} productos por agotarse ({}). ¿Quieres que envíe una alerta?", 
                                criticos.len(), 
                                criticos.iter().map(|p| p.nombre.as_str()).collect::<Vec<_>>().join(", "));
            }
            actions.push(AiAction {
                tool: "ProductoRepository::listar".into(),
                description: "Auditoría de niveles de inventario".into(),
                status: "SUCCESS".into(),
            });
            suggestions.push("Enviar alerta masiva".into());
        }
        else if query_lc.contains("alerta") || query_lc.contains("avisa") {
            // Tool: Disparar Evento en el EventBus
            EventBus::emitir(pool.clone(), tenant_id.to_string(), "ai.mass_alert", serde_json::json!({
                "message": "Alerta generada por el Asistente AI",
                "original_query": query
            })).await;

            answer = "He disparado una alerta global a través del EventBus. Tus integraciones y webhooks han sido notificados.".into();
            actions.push(AiAction {
                tool: "EventBus::emitir".into(),
                description: "Emisión de evento masivo ai.mass_alert".into(),
                status: "SUCCESS".into(),
            });
        }
        else {
            answer = "Lo siento, todavía no sé cómo ayudarte con eso. Prueba preguntándome sobre ventas, stock o alertas.".into();
            suggestions.push("¿Cuánto vendí hoy?".into());
            suggestions.push("Estado del stock".into());
        }

        Ok(AiQueryResponse {
            answer,
            actions_taken: actions,
            suggested_commands: suggestions,
        })
    }
}
