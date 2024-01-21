use axum::{routing::*, Router};

use crate::AppState;

mod healthcheck;

pub fn routes() -> impl Into<Router<AppState>> {
    axum::Router::new().route("/__healthcheck", get(healthcheck::handler))
}
