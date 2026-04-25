use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PrediccionStock {
    pub producto_id: String,
    pub nombre: String,
    pub stock_actual: i64,
    pub velocidad_diaria: f64, // unidades/día (media 7 días)
    pub dias_restantes: f64,
    pub riesgo: String, // "ALTO", "MEDIO", "ESTABLE"
}

#[derive(Serialize, ToSchema)]
pub struct ProyeccionVentas {
    pub ventas_proximos_7_dias: f64,
    pub tendencia: String, // "ALCISTA", "ESTABLE", "BAJISTA"
    pub confianza: f64, // 0.0 - 1.0 (basado en varianza)
}

#[derive(Serialize, ToSchema)]
pub struct PredictiveData {
    pub stock_en_riesgo: Vec<PrediccionStock>,
    pub forecast_ventas: ProyeccionVentas,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponsePredictive {
    pub data: PredictiveData,
}
