use sqlx::SqlitePool;
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use crate::repositories::movimiento_repository::MovimientoRepository;
use crate::repositories::producto_repository::ProductoRepository;
use crate::errors::AppError;

pub struct MovimientoService;

impl MovimientoService {
    pub async fn listar_movimientos(pool: &SqlitePool) -> Result<Vec<MovimientoInventario>, AppError> {
        let movimientos = MovimientoRepository::listar(pool).await?;
        Ok(movimientos)
    }

    pub async fn registrar_movimiento(
        pool: &SqlitePool, 
        movimiento: NuevoMovimientoDto,
        usuario_id: Option<String>
    ) -> Result<MovimientoInventario, AppError> {
        let mut tx = pool.begin().await?;

        // 1. Obtener producto y stock actual
        let producto = ProductoRepository::obtener_por_id(pool, &movimiento.producto_id).await?
            .ok_or_else(|| AppError::NotFound("Producto no encontrado".into()))?;

        let stock_antes = producto.stock_actual;
        
        // 2. Calcular nuevo stock según el tipo
        let stock_despues = match movimiento.tipo.as_str() {
            "ENTRADA" => stock_antes + movimiento.cantidad,
            "SALIDA" => {
                if stock_antes < movimiento.cantidad {
                    return Err(AppError::Conflict("Stock insuficiente para realizar la salida".into()));
                }
                stock_antes - movimiento.cantidad
            },
            "AJUSTE" => movimiento.cantidad, // En ajuste, la cantidad es el nuevo stock total o una diferencia? Hagamos que sea el nuevo stock absoluto para simplificar.
            _ => return Err(AppError::ValidationError("Tipo de movimiento inválido (ENTRADA, SALIDA, AJUSTE)".into())),
        };

        // 3. Actualizar stock del producto
        sqlx::query("UPDATE productos SET stock_actual = ?, actualizado_en = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(stock_despues)
            .bind(&producto.id)
            .execute(&mut *tx)
            .await?;

        // 4. Registrar movimiento con trazabilidad
        let reg = MovimientoRepository::registrar_transaccional(
            &mut tx, 
            movimiento, 
            stock_antes, 
            stock_despues, 
            usuario_id
        ).await?;

        tx.commit().await?;

        Ok(reg)
    }
}
