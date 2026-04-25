use sqlx::SqlitePool;
use crate::models::producto::{Producto, CrearProductoDto};
use uuid::Uuid;

pub struct ProductoRepository;

impl ProductoRepository {
    pub async fn listar(pool: &SqlitePool) -> Result<Vec<Producto>, sqlx::Error> {
        sqlx::query_as::<_, Producto>("SELECT * FROM productos ORDER BY nombre")
            .fetch_all(pool)
            .await
    }

    pub async fn obtener_por_id(pool: &SqlitePool, id: &str) -> Result<Option<Producto>, sqlx::Error> {
        sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn obtener_por_codigo(pool: &SqlitePool, codigo: &str) -> Result<Option<Producto>, sqlx::Error> {
        sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE codigo = ?")
            .bind(codigo)
            .fetch_optional(pool)
            .await
    }

    pub async fn crear(pool: &SqlitePool, dto: CrearProductoDto) -> Result<Producto, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Producto>(
            "INSERT INTO productos (id, codigo, nombre, descripcion, precio_unitario) VALUES (?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(&dto.codigo)
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(dto.precio_unitario)
        .fetch_one(pool)
        .await
    }

    pub async fn actualizar(pool: &SqlitePool, producto: &Producto) -> Result<(), sqlx::Error> {
        let ahora = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            "UPDATE productos SET codigo=?, nombre=?, descripcion=?, precio_unitario=?, actualizado_en=? WHERE id=?"
        )
        .bind(&producto.codigo)
        .bind(&producto.nombre)
        .bind(&producto.descripcion)
        .bind(producto.precio_unitario)
        .bind(&ahora)
        .bind(&producto.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn eliminar(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM productos WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
