use sqlx::SqlitePool;
use crate::models::producto::{Producto, CrearProductoDto, ActualizarProductoDto};
use uuid::Uuid;

pub struct ProductoRepository;

impl ProductoRepository {
    pub async fn listar(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<Producto>, sqlx::Error> {
        sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE tenant_id = ? ORDER BY nombre ASC")
            .bind(tenant_id)
            .fetch_all(pool)
            .await
    }

    pub async fn obtener_por_id(pool: &SqlitePool, tenant_id: &str, id: &str) -> Result<Option<Producto>, sqlx::Error> {
        sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ? AND tenant_id = ?")
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn crear(pool: &SqlitePool, tenant_id: &str, dto: CrearProductoDto) -> Result<Producto, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Producto>(
            "INSERT INTO productos (id, tenant_id, codigo, nombre, descripcion, precio_unitario, stock_actual) 
             VALUES (?, ?, ?, ?, ?, ?, 0) RETURNING *"
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(&dto.codigo)
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(dto.precio_unitario)
        .fetch_one(pool)
        .await
    }

    pub async fn actualizar(pool: &SqlitePool, tenant_id: &str, id: &str, dto: ActualizarProductoDto) -> Result<Producto, sqlx::Error> {
        sqlx::query_as::<_, Producto>(
            "UPDATE productos SET nombre = ?, descripcion = ?, precio_unitario = ?, actualizado_en = CURRENT_TIMESTAMP 
             WHERE id = ? AND tenant_id = ? RETURNING *"
        )
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(dto.precio_unitario)
        .bind(id)
        .bind(tenant_id)
        .fetch_one(pool)
        .await
    }

    pub async fn eliminar(pool: &SqlitePool, tenant_id: &str, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM productos WHERE id = ? AND tenant_id = ?")
            .bind(id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
