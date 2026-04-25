use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::autonomous::{PlatformDecision, PlatformDecisionType};
use crate::services::action_executor::ActionExecutor;
use crate::services::autonomy_ledger::AutonomyLedger;
use crate::errors::AppError;

pub struct AutonomousEngine;

impl AutonomousEngine {
    pub async fn evaluar_tenant(
        pool: &SqlitePool,
        tenant_id: &str,
    ) -> Result<Option<PlatformDecision>, AppError> {
        let ai_usage = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM platform_metrics WHERE tenant_id = ? AND categoria = 'AI'").bind(tenant_id).fetch_one(pool).await.unwrap_or(0);
        let plan = sqlx::query_scalar::<_, String>("SELECT plan_id FROM subscriptions WHERE tenant_id = ?").bind(tenant_id).fetch_one(pool).await.unwrap_or_else(|_| "BASIC".to_string());

        let decision = if plan == "BASIC" && ai_usage > 50 {
            Some(PlatformDecision {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.into(),
                decision_type: PlatformDecisionType::UpsellOpportunity,
                razon: "High AI Usage".into(),
                accion_propuesta: "ai.trigger_upsell_flow".into(),
                ejecutada: false,
                fecha: chrono::Utc::now().to_rfc3339(),
            })
        } else {
            None
        };

        if let Some(ref d) = decision {
            // 📖 LEDGER: Registrar el nacimiento de la decisión
            AutonomyLedger::registrar_inicio(pool, &d.id, &d.tenant_id, "AI_USAGE_SPIKE", "UPSELL").await?;
        }

        Ok(decision)
    }

    pub async fn procesar_decision(pool: &SqlitePool, decision: PlatformDecision) -> Result<(), AppError> {
        ActionExecutor::ejecutar(pool, decision).await?;
        Ok(())
    }
}
