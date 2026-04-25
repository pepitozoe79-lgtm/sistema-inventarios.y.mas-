use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PlatformStats {
    pub mrr: f64,
    pub arr: f64,
    pub total_tenants: i64,
    pub tenants_activos: i64,
    pub tenants_cancelados: i64,
    pub churn_rate: f64,
}

#[derive(Serialize, ToSchema)]
pub struct TenantInfo {
    pub id: String,
    pub nombre: String,
    pub plan_id: String,
    pub status: String,
    pub fecha_registro: String,
}

#[derive(Serialize, ToSchema)]
pub struct SuperAdminDashboard {
    pub stats: PlatformStats,
    pub recientes_tenants: Vec<TenantInfo>,
    pub pagos_fallidos_recientes: i64,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseSuperAdmin {
    pub data: SuperAdminDashboard,
}
