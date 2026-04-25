use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::venta::{CrearVentaRequest, DetalleVenta, Venta, VentaCompleta};
use crate::models::producto::Producto;

pub async fn crear_venta(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(venta_req): Json<CrearVentaRequest>,
) -> Result<Json<VentaCompleta>, AppError> {
    if venta_req.lineas.is_empty() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "La venta debe tener al menos una línea".into()).into());
    }

    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let venta_id = Uuid::new_v4().to_string();
    let mut total = 0.0;
    let mut detalles = Vec::new();

    for linea in &venta_req.lineas {
        // Obtener producto y verificar stock
        let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
            .bind(&linea.producto_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((axum::http::StatusCode::NOT_FOUND, format!("Producto {} no encontrado", linea.producto_id)))?;

        if producto.stock_actual < linea.cantidad {
            return Err((axum::http::StatusCode::BAD_REQUEST, format!("Stock insuficiente para {}", producto.nombre)).into());
        }

        let subtotal = producto.precio_unitario * linea.cantidad as f64;
        total += subtotal;

        // Insertar detalle
        let detalle_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO detalle_ventas (id, venta_id, producto_id, cantidad, precio_unitario, subtotal) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&detalle_id)
        .bind(&venta_id)
        .bind(&linea.producto_id)
        .bind(linea.cantidad)
        .bind(producto.precio_unitario)
        .bind(subtotal)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // Descontar stock
        let nuevo_stock = producto.stock_actual - linea.cantidad;
        sqlx::query("UPDATE productos SET stock_actual = ?, actualizado_en = datetime('now') WHERE id = ?")
            .bind(nuevo_stock)
            .bind(&linea.producto_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // Registrar movimiento de salida por venta
        let mov_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO movimientos_inventario (id, producto_id, tipo, cantidad, motivo, usuario_id) VALUES (?, ?, 'salida', ?, 'Venta', ?)"
        )
        .bind(&mov_id)
        .bind(&linea.producto_id)
        .bind(linea.cantidad)
        .bind(&claims.sub)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        detalles.push(DetalleVenta {
            id: detalle_id,
            venta_id: venta_id.clone(),
            producto_id: linea.producto_id.clone(),
            cantidad: linea.cantidad,
            precio_unitario: producto.precio_unitario,
            subtotal,
        });
    }

    // Insertar venta
    sqlx::query("INSERT INTO ventas (id, usuario_id, total) VALUES (?, ?, ?)")
        .bind(&venta_id)
        .bind(&claims.sub)
        .bind(total)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let venta = Venta {
        id: venta_id,
        usuario_id: claims.sub.clone(),
        total,
        fecha: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok(Json(VentaCompleta { venta, detalles }))
}

pub async fn listar_ventas(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Venta>>, AppError> {
    let ventas = sqlx::query_as::<_, Venta>("SELECT * FROM ventas ORDER BY fecha DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(ventas))
}
