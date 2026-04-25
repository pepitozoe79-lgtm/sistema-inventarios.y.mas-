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
            -- 1. Infraestructura Core y SaaS (Existente)
            CREATE TABLE IF NOT EXISTS tenants (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL,
                plan TEXT NOT NULL DEFAULT 'BASIC'
            );

            CREATE TABLE IF NOT EXISTS subscriptions (
                tenant_id TEXT PRIMARY KEY,
                plan_id TEXT NOT NULL DEFAULT 'BASIC',
                status TEXT NOT NULL DEFAULT 'ACTIVE'
            );

            -- 2. Auditoría de Autonomía y Gobernanza (NUEVO)
            CREATE TABLE IF NOT EXISTS autonomy_decisions (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                trigger_metric TEXT NOT NULL, -- La métrica que disparó la decisión
                decision_type TEXT NOT NULL,  -- "UPSELL", "RECOVERY", etc.
                policy_result TEXT NOT NULL,  -- "ALLOWED", "BLOCKED", "NEEDS_APPROVAL"
                action_executed TEXT,         -- La acción final tomada
                outcome TEXT,                 -- "SUCCESS", "FAILED", "PENDING_HUMAN"
                impact_measured REAL,         -- Valor numérico del impacto (ej: aumento MRR)
                fecha TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(tenant_id) REFERENCES tenants(id)
            );

            -- 3. Telemetría y Salud (Existente)
            CREATE TABLE IF NOT EXISTS platform_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tenant_id TEXT,
                categoria TEXT NOT NULL,
                metrica TEXT NOT NULL,
                valor REAL NOT NULL,
                fecha TEXT DEFAULT CURRENT_TIMESTAMP
            );

            -- 4. Negocio, Marketplace y Webhooks (Existente)
            CREATE TABLE IF NOT EXISTS integration_apps (
                id TEXT PRIMARY KEY,
                nombre TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tenant_integrations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                app_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'ACTIVE'
            );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
