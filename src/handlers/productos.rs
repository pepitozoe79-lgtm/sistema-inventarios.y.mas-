use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;
use crate::models::producto::{CrearProductoDto, ActualizarProductoDto, Producto};
use crate::models::responses::{ApiResponse, ApiListResponse};
use crate::services::producto_service::ProductoService;
use crate::errors::{AppError, ErrorResponse};

/// Listar todos los productos
#[utoipa::path(
    get,
    path = "/api/v1/productos",
    responses(
        (status = 200, description = "Lista de productos obtenida con éxito", body = ApiListResponseProducto),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn listar(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiListResponse<Producto>>, AppError> {
    let productos = ProductoService::listar_productos(&pool).await?;
    Ok(Json(ApiListResponse::new(productos)))
}

/// Obtener un producto por su ID
#[utoipa::path(
    get,
    path = "/api/v1/productos/{id}",
    params(
        ("id" = String, Path, description = "ID del producto")
    ),
    responses(
        (status = 200, description = "Producto encontrado", body = ApiResponseProducto),
        (status = 404, description = "Producto no encontrado", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn obtener(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::obtener_producto(&pool, &id).await?;
    Ok(Json(ApiResponse::new(producto)))
}

/// Crear un nuevo producto (Admin)
#[utoipa::path(
    post,
    path = "/api/v1/productos",
    request_body = CrearProductoDto,
    responses(
        (status = 201, description = "Producto creado con éxito", body = ApiResponseProducto),
        (status = 409, description = "Conflicto: El código ya existe", body = ErrorResponse),
        (status = 403, description = "Prohibido: Solo administradores", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn crear(
    State(pool): State<SqlitePool>,
    Json(dto): Json<CrearProductoDto>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::crear_producto(&pool, dto).await?;
    Ok(Json(ApiResponse::new(producto)))
}

/// Actualizar un producto existente (Admin)
#[utoipa::path(
    put,
    path = "/api/v1/productos/{id}",
    params(
        ("id" = String, Path, description = "ID del producto")
    ),
    request_body = ActualizarProductoDto,
    responses(
        (status = 200, description = "Producto actualizado con éxito", body = ApiResponseProducto),
        (status = 404, description = "Producto no encontrado", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn actualizar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(dto): Json<ActualizarProductoDto>,
) -> Result<Json<ApiResponse<Producto>>, AppError> {
    let producto = ProductoService::actualizar_producto(&pool, &id, dto).await?;
    Ok(Json(ApiResponse::new(producto)))
}

/// Eliminar un producto (Admin)
#[utoipa::path(
    delete,
    path = "/api/v1/productos/{id}",
    params(
        ("id" = String, Path, description = "ID del producto")
    ),
    responses(
        (status = 200, description = "Producto eliminado con éxito"),
        (status = 404, description = "Producto no encontrado", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn eliminar(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    ProductoService::eliminar_producto(&pool, &id).await?;
    Ok(Json(ApiResponse::new(serde_json::json!({"mensaje": "Producto eliminado"}))))
}
