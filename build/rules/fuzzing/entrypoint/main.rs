use std::path::PathBuf;

use clap::Parser;
use process_wrapper::ProcessWrapper;

#[derive(Debug, Parser)]
struct Fuzzing {
    /// Corpus.
    #[arg(long = "corpus", value_name = "DIR")]
    corpus: Option<PathBuf>,

    /// Provide a dictionary of input keywords.
    ///
    /// For some input languages using a dictionary may significantly improve
    /// the search speed.
    #[arg(long = "dict", value_name = "PATH")]
    dicts: Vec<PathBuf>,

    /// A fuzzing target.
    #[clap(flatten)]
    process_wrapper: ProcessWrapper,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    let fuzz = Fuzzing::parse();
    fuzz.process_wrapper.run().await.map_err(anyhow::Error::from)
}
