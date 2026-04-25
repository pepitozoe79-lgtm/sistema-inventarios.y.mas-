use sqlx::SqlitePool;
use crate::models::venta::{Venta, CrearVentaDto, VentaCompletaResponse};
use crate::models::inventario::NuevoMovimientoDto;
use crate::repositories::venta_repository::VentaRepository;
use crate::repositories::producto_repository::ProductoRepository;
use crate::repositories::movimiento_repository::MovimientoRepository;
use crate::services::plan_service::PlanService;
use crate::services::webhook_service::WebhookService;
use crate::errors::AppError;

pub struct VentaService;

impl VentaService {
    pub async fn listar_ventas(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<Venta>, AppError> {
        Ok(VentaRepository::listar(pool, tenant_id).await?)
    }

    pub async fn crear_venta(
        pool: &SqlitePool,
        tenant_id: &str,
        usuario_id: &str,
        dto: CrearVentaDto
    ) -> Result<VentaCompletaResponse, AppError> {
        PlanService::validar_limite_ventas(pool, tenant_id).await?;

        let mut tx = pool.begin().await?;
        let mut total_venta = 0.0;
        let mut items_preparados = Vec::new();

        for linea in &dto.lineas {
            let producto = ProductoRepository::obtener_por_id(pool, tenant_id, &linea.producto_id).await?
                .ok_or_else(|| AppError::NotFound(format!("Producto {} no encontrado", linea.producto_id)))?;

            if producto.stock_actual < linea.cantidad {
                return Err(AppError::Conflict(format!("Stock insuficiente para {}", producto.nombre)));
            }

            let subtotal = (linea.cantidad as f64) * producto.precio_unitario;
            total_venta += subtotal;
            items_preparados.push((producto, linea.cantidad, subtotal));
        }

        let venta = VentaRepository::crear_transaccional(&mut tx, tenant_id, usuario_id, total_venta).await?;
        let mut detalles = Vec::new();

        for (producto, cantidad, _subtotal) in items_preparados {
            let detalle = VentaRepository::añadir_detalle(&mut tx, tenant_id, &venta.id, &producto.id, cantidad, producto.precio_unitario).await?;
            detalles.push(detalle);

            let nuevo_stock = producto.stock_actual - cantidad;
            sqlx::query("UPDATE productos SET stock_actual = ? WHERE id = ? AND tenant_id = ?")
                .bind(nuevo_stock)
                .bind(&producto.id)
                .bind(tenant_id)
                .execute(&mut *tx)
                .await?;

            MovimientoRepository::registrar_transaccional(
                &mut tx,
                tenant_id,
                NuevoMovimientoDto {
                    producto_id: producto.id.clone(),
                    tipo: "SALIDA".into(),
                    cantidad,
                    costo_unitario: None,
                    motivo: Some(format!("Venta {}", &venta.id[0..8])),
                },
                producto.stock_actual,
                nuevo_stock,
                Some(usuario_id.into()),
            ).await?;
        }

        tx.commit().await?;

        // 📡 EVENTO: sale.created
        let pool_clone = pool.clone();
        let tenant_id_clone = tenant_id.to_string();
        let venta_clone = venta.clone();
        tokio::spawn(async move {
            WebhookService::despachar_evento(
                pool_clone,
                tenant_id_clone,
                "sale.created".into(),
                serde_json::to_value(venta_clone).unwrap(),
            ).await;
        });

        Ok(VentaCompletaResponse { venta, detalles })
    }
}
