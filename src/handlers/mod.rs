use crate::views::App;
use axum::response::{Html, IntoResponse};
use leptos::{ssr::render_to_string, *};

pub mod healthcheck;

pub async fn index() -> impl IntoResponse {
    let html = render_to_string(|| view! { <App/> });

    Html(html.to_string())
}
