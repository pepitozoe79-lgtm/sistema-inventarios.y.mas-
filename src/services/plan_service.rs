use sqlx::SqlitePool;
use crate::models::plan::{Plan, Subscription};
use crate::errors::AppError;

pub struct PlanService;

impl PlanService {
    pub async fn obtener_suscripcion(pool: &SqlitePool, tenant_id: &str) -> Result<Subscription, AppError> {
        sqlx::query_as::<_, Subscription>("SELECT * FROM subscriptions WHERE tenant_id = ?")
            .bind(tenant_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Suscripción no encontrada".into()))
    }

    pub async fn validar_limite_productos(pool: &SqlitePool, tenant_id: &str) -> Result<(), AppError> {
        let sub = Self::obtener_suscripcion(pool, tenant_id).await?;
        let plan = Plan::from_id(&sub.plan_id);

        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM productos WHERE tenant_id = ?")
            .bind(tenant_id)
            .fetch_one(pool)
            .await?;

        if count >= plan.limite_productos {
            return Err(AppError::Conflict(format!(
                "Has alcanzado el límite de productos de tu {}. ¡Mejora a PRO para ilimitados!",
                plan.nombre
            )));
        }
        Ok(())
    }

    pub async fn validar_limite_ventas(pool: &SqlitePool, tenant_id: &str) -> Result<(), AppError> {
        let sub = Self::obtener_suscripcion(pool, tenant_id).await?;
        let plan = Plan::from_id(&sub.plan_id);

        let count: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ventas WHERE tenant_id = ? AND strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now')"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await?;

        if count >= plan.limite_ventas_mes {
            return Err(AppError::Conflict(format!(
                "Has alcanzado el límite de ventas mensuales de tu {}.",
                plan.nombre
            )));
        }
        Ok(())
    }

    pub async fn validar_acceso_feature(pool: &SqlitePool, tenant_id: &str, feature: &str) -> Result<(), AppError> {
        let sub = Self::obtener_suscripcion(pool, tenant_id).await?;
        let plan = Plan::from_id(&sub.plan_id);

        let permitido = match feature {
            "BI" => plan.bi_activo,
            "PREDICTIVO" => plan.predictivo_activo,
            _ => true,
        };

        if !permitido {
            return Err(AppError::Forbidden(format!(
                "La función {} solo está disponible en el Plan PRO.",
                feature
            )));
        }
        Ok(())
    }
}
