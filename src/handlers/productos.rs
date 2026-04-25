use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{ActualizarProducto, NuevoProducto, Producto};

pub async fn listar(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Producto>>, AppError> {
    let productos = sqlx::query_as::<_, Producto>("SELECT * FROM productos ORDER BY nombre")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(productos))
}

pub async fn obtener(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Producto>, AppError> {
    let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Producto no encontrado".into()))?;
    Ok(Json(producto))
}

pub async fn crear(
    State(pool): State<SqlitePool>,
    Json(nuevo): Json<NuevoProducto>,
) -> Result<Json<Producto>, AppError> {
    let id = Uuid::new_v4().to_string();
    let producto = sqlx::query_as::<_, Producto>(
        "INSERT INTO productos (id, codigo, nombre, descripcion, precio_unitario) VALUES (?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&nuevo.codigo)
    .bind(&nuevo.nombre)
    .bind(&nuevo.descripcion)
    .bind(nuevo.precio_unitario)
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::CONFLICT, format!("Error al crear producto: {}", e)))?;
    Ok(Json(producto))
}

pub async fn actualizar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(actualizacion): Json<ActualizarProducto>,
) -> Result<Json<Producto>, AppError> {
    let mut producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Producto no encontrado".into()))?;

    if let Some(codigo) = actualizacion.codigo {
        producto.codigo = codigo;
    }
    if let Some(nombre) = actualizacion.nombre {
        producto.nombre = nombre;
    }
    if let Some(descripcion) = actualizacion.descripcion {
        producto.descripcion = Some(descripcion);
    }
    if let Some(precio) = actualizacion.precio_unitario {
        producto.precio_unitario = precio;
    }

    let ahora = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    sqlx::query(
        "UPDATE productos SET codigo=?, nombre=?, descripcion=?, precio_unitario=?, actualizado_en=? WHERE id=?"
    )
    .bind(&producto.codigo)
    .bind(&producto.nombre)
    .bind(&producto.descripcion)
    .bind(producto.precio_unitario)
    .bind(&ahora)
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    producto.actualizado_en = ahora;
    Ok(Json(producto))
}

pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let resultado = sqlx::query("DELETE FROM productos WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if resultado.rows_affected() == 0 {
        return Err((axum::http::StatusCode::NOT_FOUND, "Producto no encontrado".into()).into());
    }
    Ok(Json(serde_json::json!({"mensaje": "Producto eliminado"})))
}
