use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;
use crate::models::producto::{CrearProductoDto, ActualizarProductoDto, Producto};
use crate::services::producto_service::ProductoService;
use crate::errors::AppError;

pub async fn listar(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Producto>>, AppError> {
    let productos = ProductoService::listar_productos(&pool).await?;
    Ok(Json(productos))
}

pub async fn obtener(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Producto>, AppError> {
    let producto = ProductoService::obtener_producto(&pool, &id).await?;
    Ok(Json(producto))
}

pub async fn crear(
    State(pool): State<SqlitePool>,
    Json(dto): Json<CrearProductoDto>,
) -> Result<Json<Producto>, AppError> {
    let producto = ProductoService::crear_producto(&pool, dto).await?;
    Ok(Json(producto))
}

pub async fn actualizar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(dto): Json<ActualizarProductoDto>,
) -> Result<Json<Producto>, AppError> {
    let producto = ProductoService::actualizar_producto(&pool, &id, dto).await?;
    Ok(Json(producto))
}

pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    ProductoService::eliminar_producto(&pool, &id).await?;
    Ok(Json(serde_json::json!({"mensaje": "Producto eliminado"})))
}
