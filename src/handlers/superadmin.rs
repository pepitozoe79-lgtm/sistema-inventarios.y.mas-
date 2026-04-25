use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::models::metrics::GlobalDashboardMetrics;
use crate::services::observability_service::ObservabilityService;
use crate::errors::AppError;

/// Dashboard Global de Observabilidad para el SuperAdmin (Control Tower)
pub async fn obtener_dashboard_global(
    State(pool): State<SqlitePool>,
) -> Result<Json<GlobalDashboardMetrics>, AppError> {
    // Obtenemos la telemetría unificada de la plataforma
    let metrics = ObservabilityService::obtener_dashboard_observabilidad(&pool).await?;
    
    // Aquí podríamos disparar verificaciones de salud en tiempo real
    // (Ej: Hacer un ping a servicios externos)
    
    Ok(Json(metrics))
}
