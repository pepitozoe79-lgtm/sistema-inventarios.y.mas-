use axum::response::{IntoResponse, Response};

pub struct AppError(pub axum::http::StatusCode, pub String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl From<(axum::http::StatusCode, String)> for AppError {
    fn from(e: (axum::http::StatusCode, String)) -> Self {
        Self(e.0, e.1)
    }
}
