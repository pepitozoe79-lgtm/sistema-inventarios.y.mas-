use ax_auth::Claims;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::{header, StatusCode},
    Extension,
};
use sqlx::SqlitePool;
use crate::errors::AppError;
use crate::repositories::venta_repository::VentaRepository;
use crate::repositories::producto_repository::ProductoRepository;

use genpdf::elements::{TableLayout, Text, PaddedElement};
use genpdf::{style, Element};

pub async fn generar_factura(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Obtener la venta
    let ventas = VentaRepository::listar(&pool).await?;
    let venta = ventas.into_iter().find(|v| v.id == id)
        .ok_or_else(|| AppError::NotFound("Venta no encontrada".into()))?;

    // 2. Obtener detalles
    let detalles = VentaRepository::obtener_detalles(&pool, &id).await?;

    // 3. Crear el documento PDF
    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
        .map_err(|_| AppError::ValidationError("Error cargando fuentes".into()))?;
    
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Factura - {}", id));

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // --- ENCABEZADO ---
    doc.push(Text::new("SISTEMA DE INVENTARIO PRO").aligned(genpdf::Alignment::Center).styled(style::Style::new().bold().with_font_size(18)));
    doc.push(Text::new("NIT: 900.123.456-1").aligned(genpdf::Alignment::Center));
    doc.push(Text::new("Dirección: Calle 123 # 45-67, Ciudad").aligned(genpdf::Alignment::Center));
    doc.push(genpdf::elements::Break::new(1.5));

    // --- INFO FACTURA ---
    doc.push(Text::new(format!("FACTURA DE VENTA N°: {}", id.to_uppercase())).styled(style::Style::new().bold()));
    doc.push(Text::new(format!("Fecha: {}", venta.fecha)));
    doc.push(Text::new(format!("Cliente: Venta de Mostrador")));
    doc.push(genpdf::elements::Break::new(1));

    // --- TABLA DE PRODUCTOS ---
    let mut table = TableLayout::new(vec![1, 3, 1, 1, 1]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new());
    
    // Header de tabla
    table.row()
        .cell(Text::new("Cant").styled(style::Style::new().bold()))
        .cell(Text::new("Descripción").styled(style::Style::new().bold()))
        .cell(Text::new("Precio Unit").styled(style::Style::new().bold()))
        .cell(Text::new("Subtotal").styled(style::Style::new().bold()));

    for d in detalles {
        let producto = ProductoRepository::obtener_por_id(&pool, &d.producto_id).await?
            .map(|p| p.nombre)
            .unwrap_or_else(|| "Producto Desconocido".to_string());

        table.row()
            .cell(Text::new(format!("{}", d.cantidad)))
            .cell(Text::new(producto))
            .cell(Text::new(format!("${:.2}", d.precio_unitario)))
            .cell(Text::new(format!("${:.2}", d.subtotal)));
    }
    
    doc.push(table);
    doc.push(genpdf::elements::Break::new(1));

    // --- TOTALES ---
    doc.push(Text::new(format!("TOTAL A PAGAR: ${:.2}", venta.total))
        .aligned(genpdf::Alignment::Right)
        .styled(style::Style::new().bold().with_font_size(14)));

    doc.push(genpdf::elements::Break::new(2));
    doc.push(Text::new("Gracias por su compra.").aligned(genpdf::Alignment::Center).styled(style::Style::new().italic()));

    // 4. Renderizar a buffer
    let mut buffer = Vec::new();
    doc.render_to_write(&mut buffer)
        .map_err(|_| AppError::ValidationError("Error generando PDF".into()))?;

    // 5. Devolver respuesta
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"factura_{}.pdf\"", id)),
        ],
        buffer,
    ))
}
