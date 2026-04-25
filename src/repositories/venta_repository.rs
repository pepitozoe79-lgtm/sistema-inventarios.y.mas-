use sqlx::{SqlitePool, Sqlite, Transaction};
use crate::models::venta::{Venta, DetalleVenta};
use uuid::Uuid;

pub struct VentaRepository;

impl VentaRepository {
    pub async fn listar(pool: &SqlitePool) -> Result<Vec<Venta>, sqlx::Error> {
        sqlx::query_as::<_, Venta>("SELECT * FROM ventas ORDER BY fecha DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn crear_transaccional(
        tx: &mut Transaction<'_, Sqlite>,
        usuario_id: &str,
        total: f64,
    ) -> Result<Venta, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Venta>(
            "INSERT INTO ventas (id, usuario_id, total) VALUES (?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(usuario_id)
        .bind(total)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn añadir_detalle(
        tx: &mut Transaction<'_, Sqlite>,
        venta_id: &str,
        producto_id: &str,
        cantidad: i64,
        precio_unitario: f64,
    ) -> Result<DetalleVenta, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let subtotal = (cantidad as f64) * precio_unitario;
        sqlx::query_as::<_, DetalleVenta>(
            "INSERT INTO detalle_ventas (id, venta_id, producto_id, cantidad, precio_unitario, subtotal) 
             VALUES (?, ?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(venta_id)
        .bind(producto_id)
        .bind(cantidad)
        .bind(precio_unitario)
        .bind(subtotal)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn obtener_detalles(pool: &SqlitePool, venta_id: &str) -> Result<Vec<DetalleVenta>, sqlx::Error> {
        sqlx::query_as::<_, DetalleVenta>("SELECT * FROM detalle_ventas WHERE venta_id = ?")
            .bind(venta_id)
            .fetch_all(pool)
            .await
    }
}
