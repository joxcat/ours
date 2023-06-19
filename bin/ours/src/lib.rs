use std::net::TcpListener;

use axum::{http::StatusCode, routing::get, Router};

#[tracing::instrument]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument]
pub async fn run(listener: TcpListener) -> eyre::Result<()> {
    let router = Router::new().route("/health_check", get(health_check));

    axum::Server::from_tcp(listener)?
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

pub mod test_helpers {
    use std::{net::TcpListener, time::Duration};

    use tokio::time::sleep;

    pub async fn setup_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to listen on port");
        let port = listener.local_addr().unwrap().port();
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(super::run(listener));
        // TODO: Find why tokio need this to spawn the server before doing anything else
        sleep(Duration::from_millis(1)).await;
        format!("http://127.0.0.1:{port}")
    }
}
