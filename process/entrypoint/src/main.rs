use clap::Parser;
use entrypoint::Entrypoint;

#[derive(Clone, Debug, clap::Parser)]
struct Main {
    #[command(flatten)]
    entrypoint: Entrypoint,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    Main::parse().entrypoint.run().await.map_err(anyhow::Error::from)
}
