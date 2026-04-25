use sqlx::SqlitePool;
use crate::models::metrics::{PlatformMetric, SystemHealth, GlobalDashboardMetrics};
use crate::errors::AppError;

pub struct ObservabilityService;

impl ObservabilityService {
    /// Registra un evento de telemetría en el sistema.
    pub async fn registrar_metrica(
        pool: &SqlitePool,
        tenant_id: Option<&str>,
        categoria: &str,
        metrica: &str,
        valor: f64,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO platform_metrics (tenant_id, categoria, metrica, valor) VALUES (?, ?, ?, ?)"
        )
        .bind(tenant_id)
        .bind(categoria)
        .bind(metrica)
        .bind(valor)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Actualiza el estado de salud de un componente del sistema.
    pub async fn actualizar_salud(
        pool: &SqlitePool,
        id: &str,
        status: &str,
        latencia: Option<i32>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE system_health SET status = ?, latencia_ms = ?, actualizado_en = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(status)
        .bind(latencia)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Obtiene el Dashboard de Observabilidad Global de la plataforma.
    pub async fn obtener_dashboard_observabilidad(pool: &SqlitePool) -> Result<GlobalDashboardMetrics, AppError> {
        // 1. Calcular MRR Total (Asumiendo $29.99 por Tenant PRO activo)
        let active_pro = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM subscriptions WHERE plan_id = 'PRO' AND status = 'ACTIVE'"
        )
        .fetch_one(pool)
        .await?;
        let mrr_total = (active_pro as f64) * 29.99;

        // 2. Conteo de Tenants
        let active_tenants = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tenants").fetch_one(pool).await?;

        // 3. Telemetría Agregada (AI y EventBus)
        let ai_calls = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM platform_metrics WHERE categoria = 'AI' AND metrica = 'query'"
        )
        .fetch_one(pool)
        .await.unwrap_or(0);

        let events = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM platform_metrics WHERE categoria = 'EVENT_BUS'"
        )
        .fetch_one(pool)
        .await.unwrap_or(0);

        // 4. Salud de Componentes
        let health = sqlx::query_as::<_, SystemHealth>("SELECT * FROM system_health")
            .fetch_all(pool)
            .await?;

        Ok(GlobalDashboardMetrics {
            mrr_total,
            active_tenants,
            ai_calls_total: ai_calls,
            events_total: events,
            webhook_success_rate: 98.5, // Simulado para el MVP
            health_status: health,
        })
    }
}
