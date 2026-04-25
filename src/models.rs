use serde::{Deserialize, Serialize};

// ---------- Usuario ----------
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub rol: String,
    pub creado_en: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoUsuario {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub usuario: UsuarioPublico,
}

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct UsuarioPublico {
    pub id: String,
    pub username: String,
    pub rol: String,
}

// ---------- Producto ----------
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Producto {
    pub id: String,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_unitario: f64,
    pub stock_actual: i64,
    pub creado_en: String,
    pub actualizado_en: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoProducto {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_unitario: f64,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarProducto {
    pub codigo: Option<String>,
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub precio_unitario: Option<f64>,
}

// ---------- Movimiento de inventario ----------
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MovimientoInventario {
    pub id: String,
    pub producto_id: String,
    pub tipo: String,
    pub cantidad: i64,
    pub motivo: Option<String>,
    pub usuario_id: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoMovimiento {
    pub producto_id: String,
    pub cantidad: i64,
    pub motivo: Option<String>,
}

// ---------- Venta ----------
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Venta {
    pub id: String,
    pub usuario_id: String,
    pub total: f64,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DetalleVenta {
    pub id: String,
    pub venta_id: String,
    pub producto_id: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Deserialize)]
pub struct LineaVenta {
    pub producto_id: String,
    pub cantidad: i64,
}

#[derive(Debug, Deserialize)]
pub struct CrearVentaRequest {
    pub lineas: Vec<LineaVenta>,
}

#[derive(Debug, Serialize)]
pub struct VentaCompleta {
    pub venta: Venta,
    pub detalles: Vec<DetalleVenta>,
}
