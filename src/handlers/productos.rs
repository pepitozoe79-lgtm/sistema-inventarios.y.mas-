use axum::{
    extract::{Path, State},
    Json, Extension
};
use sqlx::SqlitePool;
use crate::models::producto::{Producto, CrearProductoDto, ActualizarProductoDto};
use crate::models::responses::{ApiResponse, ApiListResponse};
use crate::services::producto_service::ProductoService;
use crate::errors::AppError;
use crate::auth::Claims;

pub async fn listar(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<Producto>>, AppError> {
    let productos = ProductoService::listar_productos(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiListResponse::new(productos)))
}

pub async fn obtener(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::obtener_producto(&pool, &claims.tenant_id, &id).await?;
    Ok(Json(ApiResponse::new(producto)))
}

pub async fn crear(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Json(dto): Json<CrearProductoDto>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::crear_producto(&pool, &claims.tenant_id, dto).await?;
    Ok(Json(ApiResponse::new(producto)))
}

pub async fn actualizar(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(dto): Json<ActualizarProductoDto>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::actualizar_producto(&pool, &claims.tenant_id, &id, dto).await?;
    Ok(Json(ApiResponse::new(producto)))
}

pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    ProductoService::eliminar_producto(&pool, &claims.tenant_id, &id).await?;
    Ok(Json(serde_json::json!({ "mensaje": "Producto eliminado" })))
}
