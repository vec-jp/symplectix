use entrypoint::Command;
use std::os::unix::process::ExitStatusExt;

/// An entrypoint for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Run {
    #[command(flatten)]
    command: Command,
}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    // #[error("io error: {0}")]
    // Io(io::Error),
    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}

impl Run {
    pub(crate) async fn run(self) -> anyhow::Result<()> {
        use RunError::{ExitedUnsuccessfully, KilledBySignal};

        let process = self.command.spawn().await?;
        let (exit_status, _timedout) = entrypoint::wait(process).await?;

        (if exit_status.success() {
            Ok(())
        } else if let Some(code) = exit_status.code() {
            Err(ExitedUnsuccessfully(code))
        } else {
            // because `status.code()` returns `None`
            Err(KilledBySignal(exit_status.signal().expect("WIFSIGNALED is true")))
        })
        .map_err(anyhow::Error::from)
    }
}
