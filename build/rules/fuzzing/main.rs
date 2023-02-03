use clap::Parser;

mod cmd;

#[derive(Clone, Debug, Parser)]
pub enum Fuzz {
    Run(cmd::Run),
}

impl Fuzz {
    async fn run(&self) -> entrypoint::Result {
        match self {
            Fuzz::Run(f) => f.run().await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    Fuzz::parse().run().await.map_err(anyhow::Error::from)
}
