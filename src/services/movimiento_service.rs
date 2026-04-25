use sqlx::SqlitePool;
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use crate::repositories::movimiento_repository::MovimientoRepository;
use crate::repositories::producto_repository::ProductoRepository;
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

        // 1. Obtener producto (Aislado por tenant)
        let producto = ProductoRepository::obtener_por_id(pool, tenant_id, &dto.producto_id).await?
            .ok_or_else(|| AppError::NotFound("Producto no encontrado".into()))?;

        // 2. Calcular nuevo stock
        let nuevo_stock = match dto.tipo.as_str() {
            "ENTRADA" => producto.stock_actual + dto.cantidad,
            "SALIDA" => {
                if producto.stock_actual < dto.cantidad {
                    return Err(AppError::Conflict("Stock insuficiente".into()));
                }
                producto.stock_actual - dto.cantidad
            },
            "AJUSTE" => dto.cantidad, // En ajuste, la cantidad es el nuevo total
            _ => return Err(AppError::ValidationError("Tipo de movimiento inválido".into())),
        };

        // 3. Actualizar producto
        sqlx::query("UPDATE productos SET stock_actual = ?, actualizado_en = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
            .bind(nuevo_stock)
            .bind(&producto.id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        // 4. Registrar movimiento
        let movimiento = MovimientoRepository::registrar_transaccional(
            &mut tx,
            tenant_id,
            dto,
            producto.stock_actual,
            nuevo_stock,
            Some(usuario_id),
        ).await?;

        tx.commit().await?;
        Ok(movimiento)
    }
}
