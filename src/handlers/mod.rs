use axum::{routing::*, Router};

use crate::AppState;

mod healthcheck;
mod hello;

pub fn frontend_routes() -> impl Into<Router<AppState>> {
    axum::Router::new()
        .route("/__healthcheck", get(healthcheck::handler))
        .route("/hello", get(hello::handler))
}
