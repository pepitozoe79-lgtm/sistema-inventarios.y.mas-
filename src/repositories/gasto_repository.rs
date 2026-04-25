use sqlx::SqlitePool;
use crate::models::gasto::{Gasto, CrearGastoDto};
use uuid::Uuid;

pub struct GastoRepository;

impl GastoRepository {
    pub async fn listar(pool: &SqlitePool) -> Result<Vec<Gasto>, sqlx::Error> {
        sqlx::query_as::<_, Gasto>("SELECT * FROM gastos ORDER BY fecha DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn crear(pool: &SqlitePool, dto: CrearGastoDto) -> Result<Gasto, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Gasto>(
            "INSERT INTO gastos (id, tipo, monto, descripcion) VALUES (?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&dto.tipo)
        .bind(dto.monto)
        .bind(&dto.descripcion)
        .fetch_one(pool)
        .await
    }

    pub async fn eliminar(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM gastos WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
