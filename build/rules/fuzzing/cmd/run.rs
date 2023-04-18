use process::{Command, ExitStatus};

/// An entrypoint for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Run {
    #[command(flatten)]
    command: Command,
}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("the spawned child exited unsuccessfully: {0}")]
    ExitedUnsuccessfully(ExitStatus),

    #[error("the spawned child timedout: {0}")]
    Timedout(ExitStatus),
}

impl Run {
    pub(crate) async fn run(self) -> anyhow::Result<()> {
        use RunError::*;

        let process = self.command.spawn().await?;
        let (exit_status, timedout) = entrypoint::wait_and_stop(process).await?;

        (if timedout {
            Err(Timedout(exit_status))
        } else if !exit_status.success() {
            Err(ExitedUnsuccessfully(exit_status))
        } else {
            Ok(())
        })
        .map_err(anyhow::Error::from)
    }
}
