use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
    response::Json as AxumJson,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::storage::Database;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn get_connections(
    State(db): State<Arc<Database>>,
) -> Result<AxumJson<Vec<Connection>>, StatusCode> {
    let connections = db.get_recent_connections(100).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(AxumJson(connections))
}

pub async fn get_metrics() -> &'static str {
    "metrics_placeholder #TYPE counter\nconnections_total 42"
}

pub fn api_router(state: Arc<Database>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/connections", get(get_connections))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

