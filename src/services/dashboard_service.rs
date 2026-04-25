use sqlx::SqlitePool;
use crate::models::dashboard::{DashboardData, DashboardStats, TopProducto, ActividadReciente};
use crate::errors::AppError;

pub struct DashboardService;

impl DashboardService {
    pub async fn obtener_datos(pool: &SqlitePool) -> Result<DashboardData, AppError> {
        // 1. Stats generales de hoy (Financieros)
        let stats = sqlx::query_as::<_, DashboardStats>(
            r#"
            SELECT 
                COALESCE(SUM(total), 0.0) as ventas_hoy_total,
                COUNT(*) as ventas_hoy_cantidad,
                (SELECT COALESCE(SUM(cantidad), 0) FROM detalle_ventas WHERE venta_id IN (SELECT id FROM ventas WHERE date(fecha) = date('now'))) as productos_vendidos_hoy,
                (SELECT COUNT(*) FROM productos WHERE stock_actual <= 5) as alertas_stock_bajo,
                (SELECT COALESCE(SUM(monto), 0.0) FROM gastos WHERE date(fecha) = date('now')) as gastos_hoy,
                (COALESCE(SUM(total), 0.0) - (SELECT COALESCE(SUM(monto), 0.0) FROM gastos WHERE date(fecha) = date('now'))) as utilidad_hoy
            FROM ventas 
            WHERE date(fecha) = date('now')
            "#
        )
        .fetch_one(pool)
        .await?;

        // 2. Top 5 Productos
        let top_productos = sqlx::query_as::<_, TopProducto>(
            r#"
            SELECT p.nombre, SUM(dv.cantidad) as cantidad
            FROM detalle_ventas dv
            JOIN productos p ON dv.producto_id = p.id
            GROUP BY p.id
            ORDER BY cantidad DESC
            LIMIT 5
            "#
        )
        .fetch_all(pool)
        .await?;

        // 3. Actividad reciente unificada
        let actividad = sqlx::query_as::<_, ActividadReciente>(
            r#"
            SELECT 'VENTA' as tipo, ('Venta por $' || printf("%.2f", total)) as descripcion, fecha
            FROM ventas
            UNION ALL
            SELECT 'GASTO' as tipo, (tipo || ': $' || printf("%.2f", monto)) as descripcion, fecha
            FROM gastos
            UNION ALL
            SELECT 'MOVIMIENTO' as tipo, (tipo || ': ' || cantidad || ' unidades') as descripcion, fecha
            FROM movimientos_inventario
            ORDER BY fecha DESC
            LIMIT 10
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(DashboardData {
            stats,
            top_productos,
            actividad,
        })
    }
}
