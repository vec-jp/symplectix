use std::process::{ExitCode, Termination};

use anyhow::Context;

struct ExitOk(anyhow::Result<()>);

impl Termination for ExitOk {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(ref cause) => match cause.downcast_ref::<runproc::ProcStatusError>() {
                Some(err) => err.exit_code(),
                None => ExitCode::FAILURE,
            },
        }
    }
}

#[tokio::main]
async fn main() -> ExitOk {
    ExitOk(run_proc().await)
}

async fn run_proc() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    let entrypoint = runproc::Command::from_args_os(std::env::args_os());

    let status = entrypoint
        .spawn()
        .await
        .context("Failed to spawn process")?
        .wait()
        .await
        .context("Failed to fetch wait status")?;

    status.exit_ok().context("Got a failure on running the process")
}
