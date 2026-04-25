use sqlx::SqlitePool;
use crate::models::policy::{BusinessPolicy, RiskLevel};
use crate::services::plan_service::PlanService;
use crate::errors::AppError;

pub struct PolicyEngine;

impl PolicyEngine {
    /// Evalúa si una acción es permitida para un tenant específico.
    /// Cruza datos de: Plan del cliente, Nivel de Riesgo y Reglas de Negocio.
    pub async fn validar_accion(
        pool: &SqlitePool,
        tenant_id: &str,
        action_id: &str,
    ) -> Result<BusinessPolicy, AppError> {
        // 1. Obtener el plan actual del tenant
        let subscription = sqlx::query!("SELECT plan_id FROM subscriptions WHERE tenant_id = ?", tenant_id)
            .fetch_one(pool)
            .await
            .map_err(|_| AppError::NotFound("Suscripción no encontrada".into()))?;

        // 2. Definición estática de políticas (En un sistema futuro esto vendría de DB/Config)
        let policy = match action_id {
            "ai.query_sales" => BusinessPolicy {
                action_id: action_id.into(),
                min_plan: "BASIC".into(),
                risk: RiskLevel::Low,
                requires_confirmation: false,
            },
            "ai.query_stock" => BusinessPolicy {
                action_id: action_id.into(),
                min_plan: "BASIC".into(),
                risk: RiskLevel::Low,
                requires_confirmation: false,
            },
            "ai.trigger_mass_alert" => BusinessPolicy {
                action_id: action_id.into(),
                min_plan: "PRO".into(), // Solo para usuarios PRO
                risk: RiskLevel::High,
                requires_confirmation: true,
            },
            "db.delete_product" => BusinessPolicy {
                action_id: action_id.into(),
                min_plan: "BASIC".into(),
                risk: RiskLevel::High,
                requires_confirmation: true,
            },
            _ => BusinessPolicy {
                action_id: action_id.into(),
                min_plan: "BASIC".into(),
                risk: RiskLevel::Medium,
                requires_confirmation: false,
            }
        };

        // 3. Validar Plan
        if policy.min_plan == "PRO" && subscription.plan_id == "BASIC" {
            return Err(AppError::Forbidden(format!("La acción '{}' requiere el Plan PRO", action_id)));
        }

        // 4. Validar Confirmación Crítica (Simulada para acciones de alto riesgo)
        if policy.requires_confirmation && policy.risk as u8 >= RiskLevel::High as u8 {
            // En producción, aquí se generaría un 'PendingAction' para aprobación humana
            println!("⚠️ POLICY ALERT: Acción de alto riesgo detectada: {}", action_id);
        }

        Ok(policy)
    }
}
