use sqlx::SqlitePool;
use crate::models::autonomy_audit::{AutonomyDecisionRecord, AutonomyOverview};
use crate::errors::AppError;

pub struct AutonomyLedger;

impl AutonomyLedger {
    /// Registra el inicio de una decisión autónoma.
    pub async fn registrar_inicio(
        pool: &SqlitePool,
        id: &str,
        tenant_id: &str,
        trigger: &str,
        decision_type: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO autonomy_decisions (id, tenant_id, trigger_metric, decision_type, policy_result, outcome) VALUES (?, ?, ?, ?, 'PENDING_POLICY', 'PROCESSING')"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(trigger)
        .bind(decision_type)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Actualiza el resultado de la evaluación de política y el resultado final.
    pub async fn registrar_resultado(
        pool: &SqlitePool,
        id: &str,
        policy_res: &str,
        action: Option<&str>,
        outcome: &str,
        impact: Option<f64>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE autonomy_decisions SET policy_result = ?, action_executed = ?, outcome = ?, impact_measured = ? WHERE id = ?"
        )
        .bind(policy_res)
        .bind(action)
        .bind(outcome)
        .bind(impact)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Obtiene la vista de gobernanza global de la autonomía.
    pub async fn obtener_overview(pool: &SqlitePool) -> Result<AutonomyOverview, AppError> {
        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM autonomy_decisions").fetch_one(pool).await?;
        let success = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM autonomy_decisions WHERE outcome = 'SUCCESS'").fetch_one(pool).await?;
        let pending = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM autonomy_decisions WHERE outcome = 'PENDING_HUMAN'").fetch_one(pool).await?;
        
        let recent = sqlx::query_as::<_, AutonomyDecisionRecord>(
            "SELECT * FROM autonomy_decisions ORDER BY fecha DESC LIMIT 10"
        )
        .fetch_all(pool)
        .await?;

        Ok(AutonomyOverview {
            total_decisions: total,
            success_rate: if total > 0 { (success as f64 / total as f64) * 100.0 } else { 0.0 },
            pending_approvals: pending,
            total_impact_value: 0.0, // A ser calculado según métricas reales de negocio
            recent_decisions: recent,
        })
    }
}
