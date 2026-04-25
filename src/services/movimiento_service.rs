use sqlx::SqlitePool;
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use crate::repositories::movimiento_repository::MovimientoRepository;
use crate::repositories::producto_repository::ProductoRepository;
use crate::services::event_bus::EventBus;
use crate::errors::AppError;

pub struct MovimientoService;

impl MovimientoService {
    pub async fn listar_movimientos(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<MovimientoInventario>, AppError> {
        Ok(MovimientoRepository::listar(pool, tenant_id).await?)
    }

    pub async fn registrar_movimiento(
        pool: &SqlitePool,
        tenant_id: &str,
        usuario_id: String,
        dto: NuevoMovimientoDto
    ) -> Result<MovimientoInventario, AppError> {
        let mut tx = pool.begin().await?;

        let producto = ProductoRepository::obtener_por_id(pool, tenant_id, &dto.producto_id).await?
            .ok_or_else(|| AppError::NotFound("Producto no encontrado".into()))?;

        let nuevo_stock = match dto.tipo.as_str() {
            "ENTRADA" => producto.stock_actual + dto.cantidad,
            "SALIDA" => {
                if producto.stock_actual < dto.cantidad {
                    return Err(AppError::Conflict("Stock insuficiente".into()));
                }
                producto.stock_actual - dto.cantidad
            },
            "AJUSTE" => dto.cantidad,
            _ => return Err(AppError::ValidationError("Tipo de movimiento inválido".into())),
        };

        sqlx::query("UPDATE productos SET stock_actual = ?, actualizado_en = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
            .bind(nuevo_stock)
            .bind(&producto.id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let movimiento = MovimientoRepository::registrar_transaccional(
            &mut tx,
            tenant_id,
            dto,
            producto.stock_actual,
            nuevo_stock,
            Some(usuario_id),
        ).await?;

        tx.commit().await?;

        // 📡 EVENTO UNIFICADO: stock.low
        if nuevo_stock <= 5 {
            EventBus::emitir(
                pool.clone(),
                tenant_id.to_string(),
                "stock.low",
                serde_json::json!({
                    "producto_id": producto.id,
                    "nombre": producto.nombre,
                    "stock_actual": nuevo_stock
                })
            ).await;
        }

        Ok(movimiento)
    }
}
