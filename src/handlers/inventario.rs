use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::{MovimientoInventario, NuevoMovimiento};

async fn registrar_movimiento(
    pool: &SqlitePool,
    claims: &Claims,
    tipo: &str,
    movimiento: NuevoMovimiento,
) -> Result<MovimientoInventario, AppError> {
    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Verificar que el producto existe
    let producto = sqlx::query_as::<_, crate::models::Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(&movimiento.producto_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Producto no encontrado".into()))?;

    let cantidad_ajuste = if tipo == "entrada" { movimiento.cantidad } else { -movimiento.cantidad };
    let nuevo_stock = producto.stock_actual + cantidad_ajuste;
    if nuevo_stock < 0 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Stock insuficiente".into()).into());
    }

    // Actualizar stock
    sqlx::query("UPDATE productos SET stock_actual = ?, actualizado_en = datetime('now') WHERE id = ?")
        .bind(nuevo_stock)
        .bind(&movimiento.producto_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let mov = sqlx::query_as::<_, MovimientoInventario>(
        "INSERT INTO movimientos_inventario (id, producto_id, tipo, cantidad, motivo, usuario_id) VALUES (?, ?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&movimiento.producto_id)
    .bind(tipo)
    .bind(movimiento.cantidad)
    .bind(&movimiento.motivo)
    .bind(&claims.sub)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(mov)
}

pub async fn entrada(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(movimiento): Json<NuevoMovimiento>,
) -> Result<Json<MovimientoInventario>, AppError> {
    let mov = registrar_movimiento(&pool, &claims, "entrada", movimiento).await?;
    Ok(Json(mov))
}

pub async fn salida(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(movimiento): Json<NuevoMovimiento>,
) -> Result<Json<MovimientoInventario>, AppError> {
    let mov = registrar_movimiento(&pool, &claims, "salida", movimiento).await?;
    Ok(Json(mov))
}
