use std::net::TcpListener;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[tracing::instrument]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct RegisterForm {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[tracing::instrument]
async fn register(Form(payload): Form<RegisterForm>) -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument]
pub async fn run(listener: TcpListener) -> eyre::Result<()> {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/register", post(register));

    axum::Server::from_tcp(listener)?
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
