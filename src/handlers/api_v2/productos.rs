use axum::{extract::State, Json, Extension};
use sqlx::SqlitePool;
use crate::models::producto::Producto;
use crate::models::responses::ApiListResponse;
use crate::services::producto_service::ProductoService;
use crate::errors::AppError;
use crate::auth::Claims;
use axum::http::StatusCode;

/// [v2] Listar productos con validación de Scope
pub async fn listar_v2(
    State(pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiListResponse<Producto>>, AppError> {
    // 🔒 Validación de Scope
    if claims.rol == "api_key" {
        let scopes = claims.scopes.as_deref().unwrap_or("");
        if !scopes.contains("products:read") && !scopes.contains("*") {
            return Err(AppError::Forbidden("Falta scope 'products:read'".into()));
        }
    }

    let productos = ProductoService::listar_productos(&pool, &claims.tenant_id).await?;
    Ok(Json(ApiListResponse::new(productos)))
}
