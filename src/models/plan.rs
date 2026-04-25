use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Plan {
    pub id: String, // "BASIC", "PRO"
    pub nombre: String,
    pub limite_productos: i32,
    pub limite_ventas_mes: i32,
    pub bi_activo: bool,
    pub predictivo_activo: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Subscription {
    pub tenant_id: String,
    pub plan_id: String,
    pub stripe_customer_id: Option<String>,
    pub status: String, // "ACTIVE", "PAST_DUE", "CANCELED"
    pub periodo_fin: String,
}

impl Plan {
    pub fn get_basic() -> Self {
        Self {
            id: "BASIC".into(),
            nombre: "Plan Básico".into(),
            limite_productos: 50,
            limite_ventas_mes: 100,
            bi_activo: false,
            predictivo_activo: false,
        }
    }

    pub fn get_pro() -> Self {
        Self {
            id: "PRO".into(),
            nombre: "Plan Profesional".into(),
            limite_productos: 99999,
            limite_ventas_mes: 99999,
            bi_activo: true,
            predictivo_activo: true,
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "PRO" => Self::get_pro(),
            _ => Self::get_basic(),
        }
    }
}
