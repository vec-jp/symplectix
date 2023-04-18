use std::io;

use process::{Command, ExitStatus, Process};
use tokio::signal::unix::{signal, SignalKind};

#[tracing::instrument(skip(command))]
pub async fn run(command: &Command) -> io::Result<(ExitStatus, bool)> {
    let process = command.spawn().await?;
    wait_and_stop(process).await
}

#[tracing::instrument(skip(process))]
pub async fn wait_and_stop(mut process: Process) -> io::Result<(ExitStatus, bool)> {
    let mut interrupt = signal(SignalKind::interrupt())?;
    let mut terminate = signal(SignalKind::terminate())?;

    let timedout = tokio::select! {
        biased;
        _ = interrupt.recv() => { false },
        _ = terminate.recv() => { false },
        r = process.wait() => match r {
            Ok(None) => true,
            _ => false,
        },
    };

    let status = process.stop(true).await?;

    Ok((status, timedout))
}
