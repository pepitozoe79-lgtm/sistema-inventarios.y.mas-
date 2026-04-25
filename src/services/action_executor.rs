use sqlx::SqlitePool;
use crate::models::autonomous::{PlatformDecision, PlatformDecisionType};
use crate::services::policy_engine::PolicyEngine;
use crate::services::autonomy_ledger::AutonomyLedger;
use crate::services::event_bus::EventBus;
use crate::errors::AppError;

pub struct ActionExecutor;

impl ActionExecutor {
    pub async fn ejecutar(
        pool: &SqlitePool,
        decision: PlatformDecision,
    ) -> Result<(), AppError> {
        // 1. Validar gobernanza
        let policy_eval = PolicyEngine::validar_accion(pool, &decision.tenant_id, &decision.accion_propuesta).await;
        
        match policy_eval {
            Ok(p) => {
                if p.requires_confirmation {
                    // 🧾 LEDGER: Marcar como pendiente de humano
                    AutonomyLedger::registrar_resultado(pool, &decision.id, "NEEDS_APPROVAL", Some(&decision.accion_propuesta), "PENDING_HUMAN", None).await?;
                    println!("👤 HUMAN-IN-THE-LOOP: Acción '{}' en espera de aprobación.", decision.accion_propuesta);
                    return Ok(());
                }

                // Ejecución Real
                match decision.decision_type {
                    PlatformDecisionType::UpsellOpportunity => {
                        EventBus::emitir(pool.clone(), decision.tenant_id.clone(), "autonomous.upsell", serde_json::json!({ "id": decision.id })).await;
                    },
                    _ => {}
                }

                // 🧾 LEDGER: Registrar éxito
                AutonomyLedger::registrar_resultado(pool, &decision.id, "ALLOWED", Some(&decision.accion_propuesta), "SUCCESS", Some(1.0)).await?;
            },
            Err(e) => {
                // 🧾 LEDGER: Registrar bloqueo
                AutonomyLedger::registrar_resultado(pool, &decision.id, &format!("BLOCKED: {}", e), None, "DENIED", None).await?;
                return Err(AppError::Forbidden("Bloqueado por política".into()));
            }
        }

        Ok(())
    }
}
