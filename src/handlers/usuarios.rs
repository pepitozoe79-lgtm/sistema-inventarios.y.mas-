use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::usuario::{UsuarioPublico, ActualizarRol};

pub async fn listar(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<UsuarioPublico>>, AppError> {
    let usuarios = sqlx::query_as::<_, UsuarioPublico>("SELECT id, username, rol FROM usuarios ORDER BY username")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(usuarios))
}

pub async fn actualizar_rol(
    State(pool): State<SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<ActualizarRol>,
) -> Result<Json<UsuarioPublico>, AppError> {
    if payload.rol != "admin" && payload.rol != "usuario" {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Rol inválido".into()).into());
    }

    let usuario = sqlx::query_as::<_, UsuarioPublico>(
        "UPDATE usuarios SET rol = ? WHERE id = ? RETURNING id, username, rol"
    )
    .bind(&payload.rol)
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(usuario))
}
