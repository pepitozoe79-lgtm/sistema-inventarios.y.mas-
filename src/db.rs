use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:inventario.db?mode=rwc".to_string());
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            rol TEXT NOT NULL DEFAULT 'usuario',
            creado_en TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS productos (
            id TEXT PRIMARY KEY,
            codigo TEXT NOT NULL UNIQUE,
            nombre TEXT NOT NULL,
            descripcion TEXT,
            precio_unitario REAL NOT NULL,
            stock_actual INTEGER NOT NULL DEFAULT 0,
            creado_en TEXT NOT NULL DEFAULT (datetime('now')),
            actualizado_en TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS movimientos_inventario (
            id TEXT PRIMARY KEY,
            producto_id TEXT NOT NULL REFERENCES productos(id),
            tipo TEXT NOT NULL CHECK (tipo IN ('entrada', 'salida')),
            cantidad INTEGER NOT NULL,
            motivo TEXT,
            usuario_id TEXT REFERENCES usuarios(id),
            fecha TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ventas (
            id TEXT PRIMARY KEY,
            usuario_id TEXT NOT NULL REFERENCES usuarios(id),
            total REAL NOT NULL,
            fecha TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS detalle_ventas (
            id TEXT PRIMARY KEY,
            venta_id TEXT NOT NULL REFERENCES ventas(id),
            producto_id TEXT NOT NULL REFERENCES productos(id),
            cantidad INTEGER NOT NULL,
            precio_unitario REAL NOT NULL,
            subtotal REAL NOT NULL
        )"
    ).execute(pool).await?;

    Ok(())
}
