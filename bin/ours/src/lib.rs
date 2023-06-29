use std::{fs, net::TcpListener, path::PathBuf, str::FromStr};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
use tracing::log::LevelFilter;

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
    let path = "database.sqlite";
    let mut opt = ConnectOptions::new(format!("sqlite://{path}"));
    opt.sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Debug);

    if !PathBuf::from_str(path)?.exists() {
        fs::File::create(path)?;
    }
    let _db = Database::connect(opt).await?;

    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/register", post(register));

    axum::Server::from_tcp(listener)?
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
