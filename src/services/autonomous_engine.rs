use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::autonomous::{PlatformDecision, PlatformDecisionType};
use crate::services::action_executor::ActionExecutor;
use crate::errors::AppError;

pub struct AutonomousEngine;

impl AutonomousEngine {
    /// Evalúa el estado de un Tenant basándose en métricas de observabilidad.
    /// Toma decisiones automáticas para optimizar el negocio y la operación.
    pub async fn evaluar_tenant(
        pool: &SqlitePool,
        tenant_id: &str,
    ) -> Result<Option<PlatformDecision>, AppError> {
        // 1. Obtener métricas recientes del tenant
        let ai_usage = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM platform_metrics WHERE tenant_id = ? AND categoria = 'AI'"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await.unwrap_or(0);

        let plan = sqlx::query_scalar::<_, String>(
            "SELECT plan_id FROM subscriptions WHERE tenant_id = ?"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await.unwrap_or_else(|_| "BASIC".to_string());

        // --- MOTOR DE DECISIONES AUTÓNOMAS ---

        // A. Oportunidad de UPSELL: Uso excesivo de IA en plan básico
        if plan == "BASIC" && ai_usage > 50 {
            return Ok(Some(PlatformDecision {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.into(),
                decision_type: PlatformDecisionType::UpsellOpportunity,
                razon: format!("Tenant en plan BASIC ha realizado {} consultas IA.", ai_usage),
                accion_propuesta: "SUGGEST_UPGRADE_PRO".into(),
                ejecutada: false,
                fecha: chrono::Utc::now().to_rfc3339(),
            }));
        }

        // B. INTEGRATION RECOVERY: Webhooks fallando
        let webhook_failures = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM webhook_logs WHERE tenant_id = ? AND status_code >= 400"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await.unwrap_or(0);

        if webhook_failures > 10 {
            return Ok(Some(PlatformDecision {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.into(),
                decision_type: PlatformDecisionType::IntegrationRecovery,
                razon: "Alta tasa de fallos en Webhooks detectada.".into(),
                accion_propuesta: "RETRY_WITH_EXPONENTIAL_BACKOFF".into(),
                ejecutada: false,
                fecha: chrono::Utc::now().to_rfc3339(),
            }));
        }

        Ok(None)
    }

    /// Ciclo de ejecución para procesar una decisión tomada por el motor.
    pub async fn procesar_decision(
        pool: &SqlitePool,
        decision: PlatformDecision,
    ) -> Result<(), AppError> {
        ActionExecutor::ejecutar(pool, decision).await?;
        Ok(())
    }
}
