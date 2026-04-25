use axum::{extract::State, Json};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::models::responses::ApiResponse;
use crate::models::auth::RegistroDto;
use crate::auth::create_jwt;
use crate::errors::AppError;
use crate::services::provisioning_service::ProvisioningService;

#[derive(Deserialize)]
pub struct RegistroSaaSRequest {
    pub nombre_empresa: String,
    pub username_admin: String,
    pub password_admin: String,
}

#[derive(Serialize)]
pub struct RegistroSaaSResponse {
    pub tenant_id: String,
}

/// Onboarding de Negocios (SaaS Factory)
pub async fn registro_saas(
    State(pool): State<SqlitePool>,
    Json(dto): Json<RegistroSaaSRequest>,
) -> Result<Json<ApiResponse<RegistroSaaSResponse>>, AppError> {
    // Orquestar el nacimiento de la infraestructura del cliente
    let tenant_id = ProvisioningService::provisionar_nuevo_negocio(
        &pool, 
        RegistroDto {
            username: dto.username_admin,
            password: dto.password_admin,
            nombre_empresa: dto.nombre_empresa,
        }
    ).await?;

    Ok(Json(ApiResponse::new(RegistroSaaSResponse { tenant_id })))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: crate::models::usuario::Usuario,
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(dto): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let user = sqlx::query_as::<_, crate::models::usuario::Usuario>(
        "SELECT * FROM usuarios WHERE username = ?"
    )
    .bind(&dto.username)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Credenciales inválidas".into()))?;

    if bcrypt::verify(dto.password, &user.password_hash).unwrap() {
        // Resolvemos el plan del usuario para el token
        let plan_id = sqlx::query_scalar::<_, String>("SELECT plan_id FROM subscriptions WHERE tenant_id = ?")
            .bind(&user.tenant_id)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|_| "BASIC".to_string());

        let mut user_with_plan = user.clone();
        user_with_plan.rol = user.rol.clone(); // Asegurar compatibilidad
        
        // Incluimos el plan en el JWT y en la respuesta
        let token = create_jwt(&user.id, &user.tenant_id, &user.rol);
        
        Ok(Json(ApiResponse::new(LoginResponse { token, usuario: user_with_plan })))
    } else {
        Err(AppError::Unauthorized("Credenciales inválidas".into()))
    }
}
