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
            -- 1. Infraestructura y Ecosistema (Existente)
            CREATE TABLE IF NOT EXISTS tenants (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                plan TEXT NOT NULL DEFAULT 'BASIC',
                creado_en TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS subscriptions (
                tenant_id TEXT PRIMARY KEY,
                plan_id TEXT NOT NULL DEFAULT 'BASIC',
                status TEXT NOT NULL DEFAULT 'ACTIVE'
            );

            -- 2. Telemetría y Observabilidad (NUEVO)
            CREATE TABLE IF NOT EXISTS platform_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tenant_id TEXT, -- Puede ser NULL para métricas globales
                categoria TEXT NOT NULL, -- "AI", "EVENT_BUS", "BILLING", "WEBHOOK"
                metrica TEXT NOT NULL, -- "tokens_used", "event_dispatched", "success_rate"
                valor REAL NOT NULL,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS system_health (
                id TEXT PRIMARY KEY,
                componente TEXT NOT NULL, -- "DB", "EventBus", "Stripe"
                status TEXT NOT NULL, -- "UP", "DEGRADED", "DOWN"
                latencia_ms INTEGER,
                actualizado_en TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- 3. Webhooks, API Keys e Integraciones (Existente)
            CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                hashed_key TEXT NOT NULL UNIQUE,
                scopes TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS integration_apps (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                eventos_requeridos TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tenant_integrations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                app_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'ACTIVE'
            );

            -- Seeder inicial de salud del sistema
            INSERT OR IGNORE INTO system_health (id, componente, status) VALUES 
            ('db_core', 'Database Core', 'UP'),
            ('eb_central', 'Event Bus Central', 'UP'),
            ('ai_engine', 'AI Cognitive Engine', 'UP');
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
