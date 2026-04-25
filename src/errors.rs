use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Conflicto: {0}")]
    Conflict(String),

    #[error("Error interno del servidor")]
    Internal(#[from] sqlx::Error),

    #[error("No autorizado")]
    Unauthorized,

    #[error("Acceso prohibido")]
    Forbidden,
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "No autorizado".into()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Acceso prohibido".into()),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
