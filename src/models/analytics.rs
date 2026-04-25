use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PuntoSerieTemporal {
    pub fecha: String,
    pub ventas: f64,
    pub gastos: f64,
    pub utilidad: f64,
}

#[derive(Serialize, ToSchema)]
pub struct ComparativaMensual {
    pub mes_actual_total: f64,
    pub mes_pasado_total: f64,
    pub crecimiento_porcentaje: f64, // (actual - pasado) / pasado * 100
}

#[derive(Serialize, ToSchema)]
pub struct AnalyticsData {
    pub serie_30_dias: Vec<PuntoSerieTemporal>,
    pub comparativa: ComparativaMensual,
    pub ticket_promedio: f64,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseAnalytics {
    pub data: AnalyticsData,
}
