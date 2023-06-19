use std::env;

use axum::{http::StatusCode, routing::get, Router};

#[tracing::instrument]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument]
pub async fn run() -> eyre::Result<()> {
    let router = Router::new().route("/health_check", get(health_check));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("{host}:{port}").parse()?;
    tracing::debug!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

pub mod test_helpers {
    use std::time::Duration;

    use tokio::{task::JoinHandle, time::sleep};

    pub async fn setup_server() -> JoinHandle<eyre::Result<()>> {
        let handle = tokio::spawn(super::run());
        // TODO: Find why tokio need this to spawn the server before doing anything else
        sleep(Duration::from_millis(1)).await;
        handle
    }
}
