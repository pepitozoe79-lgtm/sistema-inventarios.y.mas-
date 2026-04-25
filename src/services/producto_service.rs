use sqlx::SqlitePool;
use crate::models::producto::{Producto, CrearProductoDto, ActualizarProductoDto};
use crate::repositories::producto_repository::ProductoRepository;
use crate::errors::AppError;

pub struct ProductoService;

impl ProductoService {
    pub async fn listar_productos(pool: &SqlitePool) -> Result<Vec<Producto>, AppError> {
        let productos = ProductoRepository::listar(pool).await?;
        Ok(productos)
    }

    pub async fn obtener_producto(pool: &SqlitePool, id: &str) -> Result<Producto, AppError> {
        ProductoRepository::obtener_por_id(pool, id).await?
            .ok_or_else(|| AppError::NotFound("Producto no encontrado".into()))
    }

    pub async fn crear_producto(pool: &SqlitePool, dto: CrearProductoDto) -> Result<Producto, AppError> {
        // Validación de negocio: Código único
        if let Some(_) = ProductoRepository::obtener_por_codigo(pool, &dto.codigo).await? {
            return Err(AppError::Conflict(format!("Ya existe un producto con el código {}", dto.codigo)));
        }

        let producto = ProductoRepository::crear(pool, dto).await?;
        Ok(producto)
    }

    pub async fn actualizar_producto(
        pool: &SqlitePool, 
        id: &str, 
        dto: ActualizarProductoDto
    ) -> Result<Producto, AppError> {
        let mut producto = Self::obtener_producto(pool, id).await?;

        if let Some(codigo) = dto.codigo {
            // Si el código cambia, verificar que el nuevo no exista
            if codigo != producto.codigo {
                if let Some(_) = ProductoRepository::obtener_por_codigo(pool, &codigo).await? {
                    return Err(AppError::Conflict(format!("Ya existe otro producto con el código {}", codigo)));
                }
            }
            producto.codigo = codigo;
        }

        if let Some(nombre) = dto.nombre {
            producto.nombre = nombre;
        }
        if let Some(descripcion) = dto.descripcion {
            producto.descripcion = Some(descripcion);
        }
        if let Some(precio) = dto.precio_unitario {
            producto.precio_unitario = precio;
        }

        ProductoRepository::actualizar(pool, &producto).await?;
        Ok(producto)
    }

    pub async fn eliminar_producto(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        let eliminado = ProductoRepository::eliminar(pool, id).await?;
        if !eliminado {
            return Err(AppError::NotFound("Producto no encontrado".into()));
        }
        Ok(())
    }
}
