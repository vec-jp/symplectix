use process::{Command, ExitStatus};

#[derive(Clone, Debug, clap::Parser)]
pub struct Test {
    #[command(flatten)]
    command: Command,
}

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error("the spawned child timedout: {0}")]
    Timedout(ExitStatus),

    #[error("the spawned child exited unsuccessfully: {0}")]
    ExitedWithError(ExitStatus),
}

impl Test {
    pub(crate) async fn run(self) -> anyhow::Result<()> {
        use TestError::{ExitedWithError, Timedout};

        let (exit_status, timedout) = entrypoint::run(&self.command).await?;

        (if dbg!(timedout) {
            Err(Timedout(exit_status))
        } else if exit_status.success() {
            Ok(())
        } else {
            Err(ExitedWithError(exit_status))
        })
        .map_err(anyhow::Error::from)
    }
}
