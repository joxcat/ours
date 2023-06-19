use dotenv::dotenv;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv()?;
    tracing_subscriber::Registry::default()
        .with(tracing_utils::with_env())
        .with(tracing_utils::with_hierarchical(3))
        .init();

    ours::run().await
}
