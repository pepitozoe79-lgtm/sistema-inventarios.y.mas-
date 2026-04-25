use sqlx::SqlitePool;
use crate::models::superadmin::{SuperAdminDashboard, PlatformStats, TenantInfo};
use crate::errors::AppError;

pub struct SuperAdminService;

impl SuperAdminService {
    pub async fn obtener_datos_plataforma(pool: &SqlitePool) -> Result<SuperAdminDashboard, AppError> {
        // 1. Métricas Globales
        let total_tenants = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tenants").fetch_one(pool).await?;
        let tenants_activos = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM subscriptions WHERE plan_id = 'PRO' AND status = 'ACTIVE'").fetch_one(pool).await?;
        let tenants_cancelados = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM subscriptions WHERE status = 'CANCELED'").fetch_one(pool).await?;

        // MRR Proyectado (Asumiendo Pro = $29.99/mes)
        let mrr = (tenants_activos as f64) * 29.99;
        let arr = mrr * 12.0;
        let churn_rate = if total_tenants > 0 { (tenants_cancelados as f64 / total_tenants as f64) * 100.0 } else { 0.0 };

        let stats = PlatformStats {
            mrr,
            arr,
            total_tenants,
            tenants_activos,
            tenants_cancelados,
            churn_rate,
        };

        // 2. Tenants Recientes
        let recientes_tenants = sqlx::query_as::<_, TenantInfo>(
            r#"
            SELECT t.id, t.nombre, s.plan_id, s.status, t.creado_en as fecha_registro
            FROM tenants t
            JOIN subscriptions s ON t.id = s.tenant_id
            ORDER BY t.creado_en DESC
            LIMIT 5
            "#
        )
        .fetch_all(pool)
        .await?;

        // 3. Pagos Fallidos (Simulado basado en status PAST_DUE)
        let pagos_fallidos = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM subscriptions WHERE status = 'PAST_DUE'").fetch_one(pool).await?;

        Ok(SuperAdminDashboard {
            stats,
            recientes_tenants,
            pagos_fallidos_recientes: pagos_fallidos,
        })
    }
}
