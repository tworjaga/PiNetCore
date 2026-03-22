use axum::{Router, routing::get};
use crate::storage::Database;
use std::sync::Arc;

pub fn create_api_router(db: Arc<Database>) -> Router {
    super::handlers::api_router(db)
}

