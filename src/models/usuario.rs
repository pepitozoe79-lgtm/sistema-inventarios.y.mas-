use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub rol: String,
    pub creado_en: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoUsuario {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub usuario: UsuarioPublico,
}

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct UsuarioPublico {
    pub id: String,
    pub username: String,
    pub rol: String,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarRol {
    pub rol: String,
}
