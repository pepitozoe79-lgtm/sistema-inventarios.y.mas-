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
            models::analytics::AnalyticsData,
            models::analytics::ApiResponseAnalytics,
            models::predictivo::PrediccionStock,
            models::predictivo::ProyeccionVentas,
            models::predictivo::PredictiveData,
            models::predictivo::ApiResponsePredictive,
            models::dashboard::DashboardStats,
            models::dashboard::TopProducto,
            models::dashboard::ActividadReciente,
            models::dashboard::DashboardData,
            models::dashboard::ApiResponseDashboard,
            models::responses::Meta,
            errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Predictivo", description = "Inteligencia Predictiva y Forecast"),
        (name = "Analytics", description = "BI e Inteligencia de Negocio"),
        (name = "Dashboard", description = "Dashboard en tiempo real")
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
            )
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
        .route("/api/v1/registro", post(handlers::auth_handlers::registro))
        .route("/api/v1/login", post(handlers::auth_handlers::login));

    // Rutas protegidas (JWT requerido)
    let rutas_protegidas = Router::new()
        .route("/dashboard", get(handlers::dashboard::obtener_dashboard))
        .route("/analytics", get(handlers::analytics::obtener_analytics))
        .route("/predictivo", get(handlers::predictivo::obtener_predicciones))
        .route("/productos", get(handlers::productos::listar))
        .route("/productos/:id", get(handlers::productos::obtener))
        // Rutas que requieren ser Admin
        .route("/productos", post(handlers::productos::crear).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", put(handlers::productos::actualizar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", delete(handlers::productos::eliminar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/usuarios", get(handlers::usuarios::listar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/usuarios/:id/rol", put(handlers::usuarios::actualizar_rol).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/gastos", post(handlers::gastos::crear).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/gastos/:id", delete(handlers::gastos::eliminar).route_layer(middleware::from_fn(auth::require_admin)))
        // Rutas accesibles por cualquier usuario autenticado
        .route("/gastos", get(handlers::gastos::listar))
        .route("/inventario/movimientos", get(handlers::movimientos::listar_movimientos))
        .route("/inventario/movimientos", post(handlers::movimientos::registrar))
        .route("/ventas", post(handlers::ventas::crear_venta))
        .route("/ventas", get(handlers::ventas::listar_ventas))
        .route("/ventas/:id/factura", get(handlers::facturas::generar_factura))
        .route("/reportes/inventario", get(handlers::reportes::reporte_productos))
        .route("/reportes/ventas", get(handlers::reportes::reporte_ventas));

    let app = Router::new()
        .merge(rutas_publicas)
        .nest("/api/v1", rutas_protegidas.layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware)))
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
