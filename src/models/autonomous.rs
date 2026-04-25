use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlatformDecisionType {
    UpsellOpportunity,   // Detecta que el cliente necesita un plan superior
    SystemOptimization,  // Ajuste de recursos o rate limiting
    RetentionCampaign,   // Detecta riesgo de abandono (churn)
    IntegrationRecovery, // Intento de reparación de webhooks/apps
    SelfHeal,            // Autocorrección de errores conocidos
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformDecision {
    pub id: String,
    pub tenant_id: String,
    pub decision_type: PlatformDecisionType,
    pub razon: String,
    pub accion_propuesta: String,
    pub ejecutada: bool,
    pub fecha: String,
}
