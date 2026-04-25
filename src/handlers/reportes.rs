use axum::{extract::{State, Query}, response::IntoResponse, http::header};
use sqlx::SqlitePool;
use crate::errors::AppError;
use crate::models::{Producto, Venta};
use genpdf::elements::{TableLayout, Text, PaddedElement};
use genpdf::{style, Element};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FiltrosProducto {
    pub solo_stock_bajo: Option<bool>,
    pub stock_minimo: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosVentas {
    pub mes: Option<u32>,
    pub anio: Option<i32>,
}

pub async fn reporte_productos(
    State(pool): State<SqlitePool>,
    Query(filtros): Query<FiltrosProducto>,
) -> Result<impl IntoResponse, AppError> {
    let mut query = String::from("SELECT * FROM productos WHERE 1=1");
    if filtros.solo_stock_bajo.unwrap_or(false) {
        query.push_str(&format!(" AND stock_actual <= {}", filtros.stock_minimo.unwrap_or(5)));
    }
    query.push_str(" ORDER BY nombre");

    let productos = sqlx::query_as::<_, Producto>(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Nota: genpdf requiere una fuente. Intentaremos cargar una fuente estándar si existe,
    // o fallaremos con gracia. Para producción, se debe asegurar que las fuentes existan.
    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "No se encontraron las fuentes en ./fonts (se requiere Roboto-Regular.ttf y Roboto-Bold.ttf)".to_string()))?;

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Reporte de Inventario");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(Text::new("Reporte de Inventario Actual").aligned(genpdf::Alignment::Center));
    doc.push(Text::new(format!("Fecha: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))).aligned(genpdf::Alignment::Right));
    doc.push(genpdf::elements::Break::new(1));

    let mut table = TableLayout::new(vec![2, 4, 2, 2]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new());

    table.push_row(vec![
        Text::new("Código").styled(style::Emphasis::Bold),
        Text::new("Nombre").styled(style::Emphasis::Bold),
        Text::new("Precio").styled(style::Emphasis::Bold),
        Text::new("Stock").styled(style::Emphasis::Bold),
    ]);

    for p in productos {
        table.push_row(vec![
            Text::new(p.codigo),
            Text::new(p.nombre),
            Text::new(format!("${:.2}", p.precio_unitario)),
            Text::new(p.stock_actual.to_string()),
        ]);
    }

    doc.push(table);

    let mut buffer = Vec::new();
    doc.render_to_write(&mut buffer)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        [(header::CONTENT_TYPE, "application/pdf"), 
         (header::CONTENT_DISPOSITION, "attachment; filename=\"reporte_inventario.pdf\"")],
        buffer,
    ))
}

pub async fn reporte_ventas(
    State(pool): State<SqlitePool>,
    Query(filtros): Query<FiltrosVentas>,
) -> Result<impl IntoResponse, AppError> {
    let anio = filtros.anio.unwrap_or(chrono::Local::now().format("%Y").to_string().parse().unwrap());
    let mes = filtros.mes.unwrap_or(chrono::Local::now().format("%m").to_string().parse().unwrap());
    
    let mes_str = format!("{:04}-{:02}", anio, mes);
    let ventas = sqlx::query_as::<_, Venta>("SELECT * FROM ventas WHERE fecha LIKE ? ORDER BY fecha DESC")
        .bind(format!("{}%", mes_str))
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "No se encontraron las fuentes en ./fonts".to_string()))?;

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Reporte de Ventas - {}", mes_str));

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(Text::new(format!("Reporte de Ventas Mensual - {}", mes_str)).aligned(genpdf::Alignment::Center));
    doc.push(genpdf::elements::Break::new(1));

    let mut table = TableLayout::new(vec![3, 3, 4]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new());

    table.push_row(vec![
        Text::new("Fecha").styled(style::Emphasis::Bold),
        Text::new("Total").styled(style::Emphasis::Bold),
        Text::new("ID Venta").styled(style::Emphasis::Bold),
    ]);

    let mut total_mes = 0.0;
    for v in ventas {
        total_mes += v.total;
        table.push_row(vec![
            Text::new(v.fecha),
            Text::new(format!("${:.2}", v.total)),
            Text::new(v.id.substring(0, 8).to_string()),
        ]);
    }

    doc.push(table);
    doc.push(genpdf::elements::Break::new(1));
    doc.push(Text::new(format!("TOTAL MENSUAL: ${:.2}", total_mes)).aligned(genpdf::Alignment::Right).styled(style::Emphasis::Bold));

    let mut buffer = Vec::new();
    doc.render_to_write(&mut buffer)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        [(header::CONTENT_TYPE, "application/pdf"), 
         (header::CONTENT_DISPOSITION, "attachment; filename=\"reporte_ventas.pdf\"")],
        buffer,
    ))
}
