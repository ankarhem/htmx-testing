mod blogs;
mod utils;

use axum::{routing::*, Router};

use crate::AppState;

pub fn frontend_routes() -> impl Into<Router<AppState>> {
    axum::Router::new().route("/blog", get(blogs::handler))
}
