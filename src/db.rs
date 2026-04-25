use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:inventario.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            -- 1. Infraestructura y Ecosistema Base (Existente)
            CREATE TABLE IF NOT EXISTS tenants (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                plan TEXT NOT NULL DEFAULT 'BASIC',
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS subscriptions (
                tenant_id TEXT PRIMARY KEY,
                plan_id TEXT NOT NULL DEFAULT 'BASIC',
                stripe_customer_id TEXT,
                status TEXT NOT NULL DEFAULT 'ACTIVE',
                periodo_fin TEXT NOT NULL DEFAULT '2099-12-31',
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                nombre TEXT NOT NULL,
                hashed_key TEXT NOT NULL UNIQUE,
                scopes TEXT NOT NULL,
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- 2. Marketplace de Integraciones (NUEVO)
            CREATE TABLE IF NOT EXISTS integration_apps (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                descripcion TEXT NOT NULL,
                logo_url TEXT,
                eventos_requeridos TEXT NOT NULL, -- "sale.created,product.created"
                config_schema TEXT, -- JSON Schema para la configuración
                premium INTEGER DEFAULT 0,
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS tenant_integrations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                app_id TEXT NOT NULL,
                config_json TEXT NOT NULL, -- Credenciales cifradas o tokens de la app externa
                status TEXT NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, PAUSED, ERROR
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id),
                FOREIGN KEY(app_id) REFERENCES integration_apps(id)
            );

            -- 3. Webhooks Outbound (Existente)
            CREATE TABLE IF NOT EXISTS webhook_endpoints (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                url TEXT NOT NULL,
                secret TEXT NOT NULL,
                event_types TEXT NOT NULL,
                active INTEGER DEFAULT 1,
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE TABLE IF NOT EXISTS webhook_logs (
                id TEXT PRIMARY KEY,
                endpoint_id TEXT NOT NULL,
                tenant_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                status_code INTEGER,
                request_body TEXT,
                response_body TEXT,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- 4. Negocio (Existente)
            CREATE TABLE IF NOT EXISTS productos (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                codigo TEXT NOT NULL,
                nombre TEXT NOT NULL,
                descripcion TEXT,
                precio_unitario REAL NOT NULL,
                stock_actual INTEGER DEFAULT 0,
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE TABLE IF NOT EXISTS ventas (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                usuario_id TEXT NOT NULL,
                total REAL NOT NULL,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- Seeder inicial de Apps del Marketplace
            INSERT OR IGNORE INTO integration_apps (id, nombre, descripcion, eventos_requeridos, premium)
            VALUES 
            ('shopify_sync', 'Shopify Connector', 'Sincroniza tus ventas y stock con tu tienda Shopify en tiempo real.', 'sale.created,product.updated', 1),
            ('whatsapp_notify', 'WhatsApp Alerter', 'Envía notificaciones de venta y stock bajo directamente a tu WhatsApp.', 'sale.created,stock.low', 0),
            ('google_sheets', 'Sheets Exporter', 'Exporta cada venta automáticamente a una hoja de Google Sheets.', 'sale.created', 0);
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
