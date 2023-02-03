use std::path::PathBuf;

use clap::Parser;
use process_wrapper::ProcessWrapper;

#[derive(Clone, Debug, Parser)]
pub struct Run {
    /// Custom corpus directories or artifact files.
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

impl Run {
    pub(crate) async fn run(&self) -> process_wrapper::Result {
        self.process_wrapper.run().await
    }
}
