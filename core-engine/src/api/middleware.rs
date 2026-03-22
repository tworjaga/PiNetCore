use axum::{
    middleware::Next,
    http::{Request, Response},
    response::ResponseFuture,
};
use std::future::Future;
use std::pin::Pin;

pub async fn rate_limit(req: Request<axum::body::Body>, next: Next) -> Response {
    // Rate limiting stub
    next.run(req).await
}

