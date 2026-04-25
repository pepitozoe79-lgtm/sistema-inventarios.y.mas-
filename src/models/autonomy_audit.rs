use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct AutonomyDecisionRecord {
    pub id: String,
    pub tenant_id: String,
    pub trigger_metric: String,
    pub decision_type: String,
    pub policy_result: String,
    pub action_executed: Option<String>,
    pub outcome: String,
    pub impact_measured: Option<f64>,
    pub fecha: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AutonomyOverview {
    pub total_decisions: i64,
    pub success_rate: f64,
    pub pending_approvals: i64,
    pub total_impact_value: f64,
    pub recent_decisions: Vec<AutonomyDecisionRecord>,
}
