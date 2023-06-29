use std::{env, net::TcpListener};

use dotenv::dotenv;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv()?;
    tracing_subscriber::Registry::default()
        .with(tracing_utils::with_env().and_then(tracing_utils::with_pretty()))
        .init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("{host}:{port}");
    tracing::debug!("listening on {addr}");
    let listener = TcpListener::bind(&addr)?;
    ours::run(listener).await
}
