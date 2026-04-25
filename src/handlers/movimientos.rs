use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::inventario::{MovimientoInventario, FiltrosMovimiento};

pub async fn listar_movimientos(
    State(pool): State<SqlitePool>,
    Query(filtros): Query<FiltrosMovimiento>,
) -> Result<Json<Vec<MovimientoInventario>>, AppError> {
    let mut query_str = String::from("SELECT * FROM movimientos_inventario WHERE 1=1");
    let mut binds: Vec<String> = Vec::new();

    if let Some(ref pid) = filtros.producto_id {
        query_str.push_str(" AND producto_id = ?");
        binds.push(pid.clone());
    }
    if let Some(ref tipo) = filtros.tipo {
        query_str.push_str(" AND tipo = ?");
        binds.push(tipo.clone());
    }
    if let Some(ref desde) = filtros.fecha_desde {
        query_str.push_str(" AND fecha >= ?");
        binds.push(desde.clone());
    }
    if let Some(ref hasta) = filtros.fecha_hasta {
        query_str.push_str(" AND fecha <= ?");
        binds.push(hasta.clone() + " 23:59:59");
    }
    query_str.push_str(" ORDER BY fecha DESC");

    let mut query = sqlx::query_as::<_, MovimientoInventario>(&query_str);
    for b in binds {
        query = query.bind(b);
    }

    let movimientos = query
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(movimientos))
}
