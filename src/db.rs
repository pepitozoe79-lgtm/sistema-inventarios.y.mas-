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
            -- 1. Tabla de Tenants (Empresas)
            CREATE TABLE IF NOT EXISTS tenants (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                plan TEXT NOT NULL DEFAULT 'BASIC',
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- 2. Tabla de Usuarios (aislados por tenant)
            CREATE TABLE IF NOT EXISTS usuarios (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                username TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                rol TEXT NOT NULL DEFAULT 'usuario',
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- 3. Tabla de Productos
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

            -- 4. Tabla de Ventas
            CREATE TABLE IF NOT EXISTS ventas (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                usuario_id TEXT NOT NULL,
                total REAL NOT NULL,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id),
                FOREIGN KEY(usuario_id) REFERENCES usuarios(id)
            );

            -- 5. Detalle de Ventas
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

            -- 6. Historial de Movimientos (Kardex)
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

            -- 7. Gastos
            CREATE TABLE IF NOT EXISTS gastos (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                tipo TEXT NOT NULL,
                monto REAL NOT NULL,
                descripcion TEXT,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- Índices para optimizar aislamiento
            CREATE INDEX IF NOT EXISTS idx_productos_tenant ON productos(tenant_id);
            CREATE INDEX IF NOT EXISTS idx_ventas_tenant ON ventas(tenant_id);
            CREATE INDEX IF NOT EXISTS idx_gastos_tenant ON gastos(tenant_id);
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
