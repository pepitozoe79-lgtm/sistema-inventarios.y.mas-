use axum::{
    extract::{State, Request},
    http::{HeaderMap, StatusCode},
    Json,
};
use sqlx::SqlitePool;
use serde_json::{json, Value};
use crate::models::responses::ApiResponse;
use crate::errors::AppError;
use crate::auth::Claims;
use axum::Extension;

pub async fn create_checkout_session(
    State(_pool): State<SqlitePool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, AppError> {
    // NOTA: En un entorno real usarías la librería 'stripe' oficial.
    // Aquí simulamos la creación de la URL de checkout enviando el tenant_id en metadata.
    
    let stripe_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_...".into());
    
    // Simulamos la respuesta de Stripe Checkout
    let checkout_url = format!("https://checkout.stripe.com/pay/session_...#tenant_id={}", claims.tenant_id);

    Ok(Json(json!({
        "url": checkout_url,
        "message": "Redirigiendo a pasarela de pago segura..."
    })))
}

pub async fn stripe_webhook(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, AppError> {
    let _signature = headers
        .get("stripe-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Falta firma de Stripe".into()))?;

    // 1. Validar firma (Simulado: En prod usar stripe::Webhook::construct_event)
    // 2. Parsear evento
    let event: Value = serde_json::from_str(&body).map_err(|_| AppError::ValidationError("JSON inválido".into()))?;

    let event_type = event["type"].as_str().unwrap_or("");

    match event_type {
        "checkout.session.completed" | "invoice.paid" => {
            let metadata = &event["data"]["object"]["metadata"];
            if let Some(tenant_id) = metadata["tenant_id"].as_str() {
                let stripe_customer_id = event["data"]["object"]["customer"].as_str().unwrap_or("");
                
                // 3. Ejecutar UPGRADE a PRO
                actualizar_suscripcion_pro(&pool, tenant_id, stripe_customer_id).await?;
                println!("✅ SaaS Upgrade: Tenant {} ahora es PRO", tenant_id);
            }
        }
        "customer.subscription.deleted" => {
            let metadata = &event["data"]["object"]["metadata"];
            if let Some(tenant_id) = metadata["tenant_id"].as_str() {
                // 4. Ejecutar DOWNGRADE a BASIC
                actualizar_suscripcion_basic(&pool, tenant_id).await?;
                println!("⚠️ SaaS Downgrade: Tenant {} volvió a BASIC", tenant_id);
            }
        }
        _ => {
            println!("ℹ️ Stripe Event ignorado: {}", event_type);
        }
    }

    Ok(StatusCode::OK)
}

async fn actualizar_suscripcion_pro(pool: &SqlitePool, tenant_id: &str, customer_id: &str) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE subscriptions SET plan_id = 'PRO', stripe_customer_id = ?, status = 'ACTIVE' WHERE tenant_id = ?"
    )
    .bind(customer_id)
    .bind(tenant_id)
    .execute(pool)
    .await?;
    
    // Sincronizar campo plan en la tabla tenants por conveniencia
    sqlx::query("UPDATE tenants SET plan = 'PRO' WHERE id = ?")
        .bind(tenant_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn actualizar_suscripcion_basic(pool: &SqlitePool, tenant_id: &str) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE subscriptions SET plan_id = 'BASIC', status = 'CANCELED' WHERE tenant_id = ?"
    )
    .bind(tenant_id)
    .execute(pool)
    .await?;

    sqlx::query("UPDATE tenants SET plan = 'BASIC' WHERE id = ?")
        .bind(tenant_id)
        .execute(pool)
        .await?;

    Ok(())
}
