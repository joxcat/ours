#![cfg_attr(tarpaulin, ignore)]

use std::{net::TcpListener, time::Duration};

use tokio::time::sleep;
use tracing_subscriber::fmt::format::FmtSpan;

pub async fn setup_server() -> String {
    setup_tracing();
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to listen on port");
    let port = listener.local_addr().unwrap().port();
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(ours::run(listener));
    // TODO: Find why tokio need this to spawn the server before doing anything else
    sleep(Duration::from_millis(1)).await;
    format!("http://127.0.0.1:{port}")
}

pub fn setup_tracing() {
    tracing_subscriber::fmt::fmt()
        .compact()
        .with_test_writer()
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter("debug")
        .init();
}
