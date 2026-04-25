use sqlx::SqlitePool;
use crate::models::predictivo::{PredictiveData, PrediccionStock, ProyeccionVentas};
use crate::errors::AppError;

pub struct PredictiveService;

impl PredictiveService {
    pub async fn generar_proyecciones(pool: &SqlitePool) -> Result<PredictiveData, AppError> {
        // 1. Predicción de Stock (Run-rate)
        // Calculamos velocidad diaria de los últimos 30 días
        let stock_en_riesgo = sqlx::query_as::<_, PrediccionStock>(
            r#"
            SELECT 
                p.id as producto_id,
                p.nombre,
                p.stock_actual,
                COALESCE((SELECT SUM(cantidad) FROM detalle_ventas dv JOIN ventas v ON dv.venta_id = v.id WHERE dv.producto_id = p.id AND v.fecha >= date('now', '-30 days')), 0) / 30.0 as velocidad_diaria,
                CASE 
                    WHEN (SELECT SUM(cantidad) FROM detalle_ventas dv JOIN ventas v ON dv.venta_id = v.id WHERE dv.producto_id = p.id AND v.fecha >= date('now', '-30 days')) > 0 
                    THEN p.stock_actual / (COALESCE((SELECT SUM(cantidad) FROM detalle_ventas dv JOIN ventas v ON dv.venta_id = v.id WHERE dv.producto_id = p.id AND v.fecha >= date('now', '-30 days')), 0) / 30.0)
                    ELSE 999 
                END as dias_restantes,
                CASE 
                    WHEN p.stock_actual / (COALESCE((SELECT SUM(cantidad) FROM detalle_ventas dv JOIN ventas v ON dv.venta_id = v.id WHERE dv.producto_id = p.id AND v.fecha >= date('now', '-30 days')), 0) / 30.0) <= 3 THEN 'ALTO'
                    WHEN p.stock_actual / (COALESCE((SELECT SUM(cantidad) FROM detalle_ventas dv JOIN ventas v ON dv.venta_id = v.id WHERE dv.producto_id = p.id AND v.fecha >= date('now', '-30 days')), 0) / 30.0) <= 7 THEN 'MEDIO'
                    ELSE 'ESTABLE'
                END as riesgo
            FROM productos p
            WHERE velocidad_diaria > 0
            ORDER BY dias_restantes ASC
            LIMIT 10
            "#
        )
        .fetch_all(pool)
        .await?;

        // 2. Forecast de Ventas (Tendencia simple de 7 días)
        let ventas_7_dias_reales = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(SUM(total), 0.0) FROM ventas WHERE fecha >= date('now', '-7 days')"
        ).fetch_one(pool).await?;

        let ventas_7_dias_previos = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(SUM(total), 0.0) FROM ventas WHERE fecha >= date('now', '-14 days') AND fecha < date('now', '-7 days')"
        ).fetch_one(pool).await?;

        let tendencia = if ventas_7_dias_reales > ventas_7_dias_previos {
            "ALCISTA".to_string()
        } else if ventas_7_dias_reales < ventas_7_dias_previos {
            "BAJISTA".to_string()
        } else {
            "ESTABLE".to_string()
        };

        // Proyección simple: si es alcista, sumamos el crecimiento porcentual
        let multiplicador = if ventas_7_dias_previos > 0.0 {
            1.0 + ((ventas_7_dias_reales - ventas_7_dias_previos) / ventas_7_dias_previos)
        } else {
            1.0
        };

        let forecast_ventas = ProyeccionVentas {
            ventas_proximos_7_dias: ventas_7_dias_reales * multiplicador.clamp(0.5, 1.5),
            tendencia,
            confianza: 0.85, // Heurística fija por ahora
        };

        Ok(PredictiveData {
            stock_en_riesgo,
            forecast_ventas,
        })
    }
}
