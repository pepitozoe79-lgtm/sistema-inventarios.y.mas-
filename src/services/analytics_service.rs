use sqlx::SqlitePool;
use crate::models::analytics::{AnalyticsData, PuntoSerieTemporal, ComparativaMensual};
use crate::errors::AppError;

pub struct AnalyticsService;

impl AnalyticsService {
    pub async fn obtener_analytics(pool: &SqlitePool, tenant_id: &str) -> Result<AnalyticsData, AppError> {
        // 1. Serie Temporal - Últimos 30 días (Aislado por tenant)
        let serie_30_dias = sqlx::query_as::<_, PuntoSerieTemporal>(
            r#"
            WITH RECURSIVE dias(fecha) AS (
                SELECT date('now', '-29 days')
                UNION ALL
                SELECT date(fecha, '+1 day') FROM dias WHERE fecha < date('now')
            )
            SELECT 
                d.fecha,
                COALESCE((SELECT SUM(total) FROM ventas WHERE tenant_id = ? AND date(fecha) = d.fecha), 0.0) as ventas,
                COALESCE((SELECT SUM(monto) FROM gastos WHERE tenant_id = ? AND date(fecha) = d.fecha), 0.0) as gastos,
                (COALESCE((SELECT SUM(total) FROM ventas WHERE tenant_id = ? AND date(fecha) = d.fecha), 0.0) - 
                 COALESCE((SELECT SUM(monto) FROM gastos WHERE tenant_id = ? AND date(fecha) = d.fecha), 0.0)) as utilidad
            FROM dias d
            ORDER BY d.fecha ASC
            "#
        )
        .bind(tenant_id)
        .bind(tenant_id)
        .bind(tenant_id)
        .bind(tenant_id)
        .fetch_all(pool)
        .await?;

        // 2. Comparativa Mensual
        let mes_actual = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(SUM(total), 0.0) FROM ventas WHERE tenant_id = ? AND strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now')"
        ).bind(tenant_id).fetch_one(pool).await?;

        let mes_pasado = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(SUM(total), 0.0) FROM ventas WHERE tenant_id = ? AND strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now', '-1 month')"
        ).bind(tenant_id).fetch_one(pool).await?;

        let crecimiento = if mes_pasado > 0.0 { ((mes_actual - mes_pasado) / mes_pasado) * 100.0 } else { 0.0 };

        // 3. Ticket Promedio Histórico
        let ticket_promedio = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(AVG(total), 0.0) FROM ventas WHERE tenant_id = ?"
        ).bind(tenant_id).fetch_one(pool).await?;

        Ok(AnalyticsData {
            serie_30_dias,
            comparativa: ComparativaMensual { mes_actual_total: mes_actual, mes_pasado_total: mes_pasado, crecimiento_porcentaje: crecimiento },
            ticket_promedio,
        })
    }
}
