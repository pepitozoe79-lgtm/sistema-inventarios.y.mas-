mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod repositories;
mod services;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::productos::listar,
        handlers::productos::obtener,
        handlers::productos::crear,
        handlers::productos::actualizar,
        handlers::productos::eliminar,
        handlers::movimientos::listar_movimientos,
        handlers::movimientos::registrar,
        handlers::ventas::listar_ventas,
        handlers::ventas::crear_venta,
        handlers::dashboard::obtener_dashboard,
        handlers::gastos::listar,
        handlers::gastos::crear,
        handlers::gastos::eliminar,
        handlers::analytics::obtener_analytics,
        handlers::predictivo::obtener_predicciones,
        handlers::superadmin::obtener_dashboard_global,
        handlers::api_v2::productos::listar_v2,
        handlers::api_v2::ai::query_ai,
        handlers::integrations::listar_marketplace,
    ),
    components(
        schemas(
            models::producto::Producto,
            models::producto::CrearProductoDto,
            models::producto::ActualizarProductoDto,
            models::producto::ApiResponseProducto,
            models::producto::ApiListResponseProducto,
            models::inventario::MovimientoInventario,
            models::inventario::NuevoMovimientoDto,
            models::inventario::ApiResponseMovimiento,
            models::inventario::ApiListResponseMovimiento,
            models::venta::Venta,
            models::venta::DetalleVenta,
            models::venta::CrearVentaDto,
            models::venta::VentaCompletaResponse,
            models::venta::ApiResponseVenta,
            models::venta::ApiListResponseVenta,
            models::gasto::Gasto,
            models::gasto::CrearGastoDto,
            models::gasto::ApiResponseGasto,
            models::gasto::ApiListResponseGasto,
            models::superadmin::PlatformStats,
            models::superadmin::TenantInfo,
            models::superadmin::SuperAdminDashboard,
            models::superadmin::ApiResponseSuperAdmin,
            models::analytics::AnalyticsData,
            models::analytics::ApiResponseAnalytics,
            models::predictivo::PredictiveData,
            models::predictivo::ApiResponsePredictive,
            models::dashboard::DashboardStats,
            models::dashboard::TopProducto,
            models::dashboard::ActividadReciente,
            models::dashboard::DashboardData,
            models::dashboard::ApiResponseDashboard,
            models::api_key::ApiKey,
            models::api_key::CrearApiKeyDto,
            models::api_key::ApiKeyGeneradaResponse,
            models::webhook::WebhookEndpoint,
            models::webhook::CrearWebhookDto,
            models::webhook::WebhookLog,
            models::integration::IntegrationApp,
            models::integration::TenantIntegration,
            models::integration::InstalarAppDto,
            models::integration::IntegrationAppFull,
            models::ai::AiQueryRequest,
            models::ai::AiQueryResponse,
            models::ai::AiAction,
            models::responses::Meta,
            errors::ErrorResponse,
        )
    ),
    tags(
        (name = "IA", description = "Asistente de Negocio Cognitivo"),
        (name = "Ecosistema", description = "Marketplace de Aplicaciones"),
        (name = "Integración", description = "Webhooks y API Pública")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "api_key",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header("X-API-Key".to_string())
                ),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let pool = db::init_db().await.expect("No se pudo conectar a la DB");
    db::run_migrations(&pool).await.unwrap();

    // Rutas públicas
    let rutas_publicas = Router::new()
        .route("/api/v1/health", get(|| async { "OK" }))
        .route("/api/v1/registro", post(handlers::auth_handlers::registro_saas))
        .route("/api/v1/login", post(handlers::auth_handlers::login))
        .route("/api/v1/billing/webhook", post(handlers::billing::stripe_webhook));

    // Rutas de SuperAdmin
    let rutas_superadmin = Router::new()
        .route("/superadmin/dashboard", get(handlers::superadmin::obtener_dashboard_global))
        .layer(middleware::from_fn(auth::require_superadmin));

    // Rutas de Configuración y Ecosistema (v1 - Internas)
    let rutas_ecosistema = Router::new()
        .route("/settings/api-keys", get(handlers::api_keys::listar_keys))
        .route("/settings/api-keys", post(handlers::api_keys::crear_key))
        .route("/settings/api-keys/:id", delete(handlers::api_keys::eliminar_key))
        .route("/settings/webhooks", get(handlers::webhooks::listar_endpoints))
        .route("/settings/webhooks", post(handlers::webhooks::crear_endpoint))
        .route("/settings/webhooks/logs", get(handlers::webhooks::listar_logs))
        .route("/settings/webhooks/:id", delete(handlers::webhooks::eliminar_endpoint))
        .route("/ecosistema/marketplace", get(handlers::integrations::listar_marketplace))
        .route("/ecosistema/marketplace/instalar", post(handlers::integrations::instalar_app))
        .route("/ecosistema/marketplace/:app_id", delete(handlers::integrations::desinstalar_app));

    // Rutas Públicas v2 (Integraciones + IA)
    let rutas_v2 = Router::new()
        .route("/products", get(handlers::api_v2::productos::listar_v2))
        .route("/ai/query", post(handlers::api_v2::ai::query_ai));

    // Rutas protegidas por Tenant (v1)
    let rutas_protegidas = Router::new()
        .route("/dashboard", get(handlers::dashboard::obtener_dashboard))
        .route("/analytics", get(handlers::analytics::obtener_analytics))
        .route("/predictivo", get(handlers::predictivo::obtener_predicciones))
        .route("/billing/checkout", post(handlers::billing::create_checkout_session))
        .route("/productos", get(handlers::productos::listar))
        .route("/productos/:id", get(handlers::productos::obtener))
        .route("/productos", post(handlers::productos::crear).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", put(handlers::productos::actualizar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", delete(handlers::productos::eliminar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/inventario/movimientos", get(handlers::movimientos::listar_movimientos))
        .route("/inventario/movimientos", post(handlers::movimientos::registrar))
        .route("/ventas", post(handlers::ventas::crear_venta))
        .route("/ventas", get(handlers::ventas::listar_ventas))
        .merge(rutas_ecosistema);

    let app = Router::new()
        .merge(rutas_publicas)
        .nest("/api/v1", rutas_superadmin)
        .nest("/api/v1", rutas_protegidas.layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware)))
        .nest("/api/v2", rutas_v2.layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware)))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Servidor corriendo en http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
