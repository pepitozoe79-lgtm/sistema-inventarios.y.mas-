use sqlx::SqlitePool;
use crate::models::gasto::{Gasto, CrearGastoDto};
use uuid::Uuid;

pub struct GastoRepository;

impl GastoRepository {
    pub async fn listar(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<Gasto>, sqlx::Error> {
        sqlx::query_as::<_, Gasto>("SELECT * FROM gastos WHERE tenant_id = ? ORDER BY fecha DESC")
            .bind(tenant_id)
            .fetch_all(pool)
            .await
    }

    pub async fn crear(pool: &SqlitePool, tenant_id: &str, dto: CrearGastoDto) -> Result<Gasto, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Gasto>(
            "INSERT INTO gastos (id, tenant_id, tipo, monto, descripcion) VALUES (?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(&dto.tipo)
        .bind(dto.monto)
        .bind(&dto.descripcion)
        .fetch_one(pool)
        .await
    }

    pub async fn eliminar(pool: &SqlitePool, tenant_id: &str, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM gastos WHERE id = ? AND tenant_id = ?")
            .bind(id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
