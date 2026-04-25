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
            -- 1. Tenants y Suscripciones (Existente)
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

            -- 2. API Keys para Integraciones Externas (NUEVO)
            CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                nombre TEXT NOT NULL,
                hashed_key TEXT NOT NULL UNIQUE,
                scopes TEXT NOT NULL, -- JSON o string separado por comas: "products:read,sales:write"
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                ultima_vez_usada TEXT,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- 3. Usuarios, Productos, Ventas, etc. (Existente)
            CREATE TABLE IF NOT EXISTS usuarios (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                username TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                rol TEXT NOT NULL DEFAULT 'usuario',
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE TABLE IF NOT EXISTS productos (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                codigo TEXT NOT NULL,
                nombre TEXT NOT NULL,
                descripcion TEXT,
                precio_unitario REAL NOT NULL,
                stock_actual INTEGER DEFAULT 0,
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                actualizado_en TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE TABLE IF NOT EXISTS ventas (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                usuario_id TEXT NOT NULL,
                total REAL NOT NULL,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id),
                FOREIGN KEY(usuario_id) REFERENCES usuarios(id)
            );

            CREATE TABLE IF NOT EXISTS detalle_ventas (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                venta_id TEXT NOT NULL,
                producto_id TEXT NOT NULL,
                cantidad INTEGER NOT NULL,
                precio_unitario REAL NOT NULL,
                subtotal REAL NOT NULL,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id),
                FOREIGN KEY(venta_id) REFERENCES ventas(id),
                FOREIGN KEY(producto_id) REFERENCES productos(id)
            );

            CREATE TABLE IF NOT EXISTS movimientos_inventario (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                producto_id TEXT NOT NULL,
                usuario_id TEXT,
                tipo TEXT NOT NULL,
                cantidad INTEGER NOT NULL,
                stock_antes INTEGER NOT NULL,
                stock_despues INTEGER NOT NULL,
                costo_unitario REAL,
                motivo TEXT,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id),
                FOREIGN KEY(producto_id) REFERENCES productos(id)
            );

            CREATE TABLE IF NOT EXISTS gastos (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                tipo TEXT NOT NULL,
                monto REAL NOT NULL,
                descripcion TEXT,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            CREATE INDEX IF NOT EXISTS idx_api_keys_tenant ON api_keys(tenant_id);
            CREATE INDEX IF NOT EXISTS idx_productos_tenant ON productos(tenant_id);
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
