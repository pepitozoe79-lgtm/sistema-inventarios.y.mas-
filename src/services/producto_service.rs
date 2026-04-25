use sqlx::SqlitePool;
use crate::models::producto::{Producto, CrearProductoDto, ActualizarProductoDto};
use crate::repositories::producto_repository::ProductoRepository;
use crate::errors::AppError;

pub struct ProductoService;

impl ProductoService {
    pub async fn listar_productos(pool: &SqlitePool, tenant_id: &str) -> Result<Vec<Producto>, AppError> {
        Ok(ProductoRepository::listar(pool, tenant_id).await?)
    }

    pub async fn obtener_producto(pool: &SqlitePool, tenant_id: &str, id: &str) -> Result<Producto, AppError> {
        ProductoRepository::obtener_por_id(pool, tenant_id, id).await?
            .ok_or_else(|| AppError::NotFound("Producto no encontrado".into()))
    }

    pub async fn crear_producto(pool: &SqlitePool, tenant_id: &str, dto: CrearProductoDto) -> Result<Producto, AppError> {
        // Validación de duplicados por código dentro del mismo tenant
        let existentes = ProductoRepository::listar(pool, tenant_id).await?;
        if existentes.iter().any(|p| p.codigo == dto.codigo) {
            return Err(AppError::Conflict("El código de producto ya existe en tu empresa".into()));
        }
        Ok(ProductoRepository::crear(pool, tenant_id, dto).await?)
    }

    pub async fn actualizar_producto(pool: &SqlitePool, tenant_id: &str, id: &str, dto: ActualizarProductoDto) -> Result<Producto, AppError> {
        Ok(ProductoRepository::actualizar(pool, tenant_id, id, dto).await?)
    }

    pub async fn eliminar_producto(pool: &SqlitePool, tenant_id: &str, id: &str) -> Result<(), AppError> {
        Ok(ProductoRepository::eliminar(pool, tenant_id, id).await?)
    }
}
