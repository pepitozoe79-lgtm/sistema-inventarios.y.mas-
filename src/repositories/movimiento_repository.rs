use sqlx::{SqlitePool, Sqlite, Transaction};
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use uuid::Uuid;

pub struct MovimientoRepository;

impl MovimientoRepository {
    pub async fn listar(pool: &SqlitePool) -> Result<Vec<MovimientoInventario>, sqlx::Error> {
        sqlx::query_as::<_, MovimientoInventario>("SELECT * FROM movimientos_inventario ORDER BY fecha DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn registrar_transaccional(
        tx: &mut Transaction<'_, Sqlite>,
        movimiento: NuevoMovimientoDto,
        stock_antes: i64,
        stock_despues: i64,
        usuario_id: Option<String>,
    ) -> Result<MovimientoInventario, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, MovimientoInventario>(
            "INSERT INTO movimientos_inventario (id, producto_id, tipo, cantidad, stock_antes, stock_despues, costo_unitario, motivo, usuario_id) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&movimiento.producto_id)
        .bind(&movimiento.tipo)
        .bind(movimiento.cantidad)
        .bind(stock_antes)
        .bind(stock_despues)
        .bind(movimiento.costo_unitario)
        .bind(&movimiento.motivo)
        .bind(usuario_id)
        .fetch_one(&mut **tx)
        .await
    }
}
