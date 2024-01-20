use std::net::TcpListener;

use anyhow::Result;
use htmx_testing::telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber =
        telemetry::get_subscriber("htmx_testing".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
    htmx_testing::run(listener).await?;

    Ok(())
}
