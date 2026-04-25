use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskLevel {
    Low,      // Lectura de datos, consultas simples
    Medium,   // Creación de registros, actualizaciones menores
    High,     // Acciones masivas, eliminación, cambios financieros
    Critical, // Cambios de plan, acceso SuperAdmin, borrado total
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusinessPolicy {
    pub action_id: String,
    pub min_plan: String, // "BASIC", "PRO"
    pub risk: RiskLevel,
    pub requires_confirmation: bool,
}
