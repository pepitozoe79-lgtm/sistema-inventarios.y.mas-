use sqlx::SqlitePool;
use crate::models::autonomous::{PlatformDecision, PlatformDecisionType};
use crate::services::policy_engine::PolicyEngine;
use crate::services::event_bus::EventBus;
use crate::errors::AppError;

pub struct ActionExecutor;

impl ActionExecutor {
    /// Ejecuta una decisión autónoma validando siempre con el Policy Engine.
    pub async fn ejecutar(
        pool: &SqlitePool,
        decision: PlatformDecision,
    ) -> Result<(), AppError> {
        println!("⚡ EXECUTOR: Procesando decisión autónoma: {} para Tenant {}", decision.accion_propuesta, decision.tenant_id);

        // 1. Validar gobernanza antes de cualquier ejecución automática
        match PolicyEngine::validar_accion(pool, &decision.tenant_id, &decision.accion_propuesta).await {
            Ok(_) => {
                // Proceder con la ejecución según el tipo
                match decision.decision_type {
                    PlatformDecisionType::UpsellOpportunity => {
                        // Acción: Notificar al AI Copilot que sugiera un upgrade
                        EventBus::emitir(
                            pool.clone(),
                            decision.tenant_id.clone(),
                            "autonomous.upsell_suggestion",
                            serde_json::json!({ "reason": decision.razon })
                        ).await;
                    },
                    PlatformDecisionType::IntegrationRecovery => {
                        // Acción: Activar modo de reintento inteligente
                        println!("🔧 Recovery: Ajustando política de reintentos para Tenant {}", decision.tenant_id);
                    },
                    _ => {
                        println!("⚠️ Executor: Tipo de decisión no implementado aún.");
                    }
                }
            },
            Err(e) => {
                println!("🛑 POLICY BLOCK: El ejecutor autónomo fue bloqueado: {}", e);
                return Err(AppError::Forbidden("Acción autónoma bloqueada por política de seguridad".into()));
            }
        }

        Ok(())
    }
}
