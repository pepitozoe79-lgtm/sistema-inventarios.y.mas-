use sqlx::{Sqlite, Transaction, SqlitePool};
use crate::models::inventario::{MovimientoInventario, NuevoMovimientoDto};
use uuid::Uuid;

pub struct MovimientoRepository;

impl MovimientoRepository {
    pub async fn listar(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<MovimientoInventario>, sqlx::Error> {
        sqlx::query_as::<_, MovimientoInventario>(
            "SELECT * FROM movimientos_inventario WHERE tenant_id = ? ORDER BY fecha DESC"
        )
        .bind(tenant_id)
        .fetch_all(pool)
        .await
    }

    pub async fn registrar_transaccional(
        tx: &mut Transaction<'_, Sqlite>,
        tenant_id: &str,
        dto: NuevoMovimientoDto,
        stock_antes: i64,
        stock_despues: i64,
        usuario_id: Option<String>,
    ) -> Result<MovimientoInventario, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, MovimientoInventario>(
            r#"
            INSERT INTO movimientos_inventario 
            (id, tenant_id, producto_id, usuario_id, tipo, cantidad, stock_antes, stock_despues, costo_unitario, motivo)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(&dto.producto_id)
        .bind(usuario_id)
        .bind(&dto.tipo)
        .bind(dto.cantidad)
        .bind(stock_antes)
        .bind(stock_despues)
        .bind(dto.costo_unitario)
        .bind(&dto.motivo)
        .fetch_one(&mut **tx)
        .await
    }
}
