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
    ),
    components(
        schemas(
            models::producto::Producto,
            models::producto::CrearProductoDto,
            models::producto::ActualizarProductoDto,
        )
    ),
    tags(
        (name = "Productos", description = "Gestión de productos e inventario")
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
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
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
        .route("/api/registro", post(handlers::auth_handlers::registro))
        .route("/api/login", post(handlers::auth_handlers::login));

    // Rutas protegidas (JWT requerido)
    let rutas_protegidas = Router::new()
        .route("/productos", get(handlers::productos::listar))
        .route("/productos/:id", get(handlers::productos::obtener))
        // Rutas que requieren ser Admin
        .route("/productos", post(handlers::productos::crear).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", put(handlers::productos::actualizar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/productos/:id", delete(handlers::productos::eliminar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/usuarios", get(handlers::usuarios::listar).route_layer(middleware::from_fn(auth::require_admin)))
        .route("/usuarios/:id/rol", put(handlers::usuarios::actualizar_rol).route_layer(middleware::from_fn(auth::require_admin)))
        // Rutas accesibles por cualquier usuario autenticado
        .route("/inventario/movimientos", get(handlers::movimientos::listar_movimientos))
        .route("/ventas", post(handlers::ventas::crear_venta))
        .route("/ventas", get(handlers::ventas::listar_ventas))
        .route("/reportes/inventario", get(handlers::reportes::reporte_productos))
        .route("/reportes/ventas", get(handlers::reportes::reporte_ventas))
        .layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware));

    let app = Router::new()
        .merge(rutas_publicas)
        .nest("/api", rutas_protegidas)
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
