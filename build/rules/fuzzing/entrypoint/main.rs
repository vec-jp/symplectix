use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct EntrypointOptions {
    #[arg(long = "corpus", value_name = "DIR")]
    corpus: Option<PathBuf>,

    #[clap(flatten)]
    process_wrapper_options: process_wrapper::Options,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    let opts = EntrypointOptions::parse();
    process_wrapper::run(&opts.process_wrapper_options).await.map_err(anyhow::Error::from)
}
