use std::io;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::time::Duration;

use clap::Parser;
use futures::future;
use futures::prelude::*;
use tokio::process::{Child, Command};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;

mod fsutil;
use Error::*;

pub fn parse_args() -> Options {
    Options::parse()
}

/// ProcessWrapper executes a subprocess.
#[derive(Debug, Clone, Parser)]
pub struct Options {
    /// List of paths to wait for before spawning the child process.
    #[arg(long = "wait-file", value_name = "PATH")]
    wait_files: Vec<PathBuf>,

    /// Create a file after the child process exits successfully.
    #[arg(long, value_name = "PATH")]
    post_file: Option<PathBuf>,

    /// Redirect the child process stdout.
    #[arg(long, value_name = "PATH")]
    stdout: Option<PathBuf>,

    /// Redirect the child process stderr.
    #[arg(long, value_name = "PATH")]
    stderr: Option<PathBuf>,

    /// Kill the spawned child process after the specified duration.
    /// The timeout clock does not tick until the child spawns.
    /// So the operations before spawning, for example waiting for `wait-file`s, never times out.
    #[arg(long, value_name = "DURATION")]
    timeout: Option<humantime::Duration>,

    /// The entrypoint of the child process.
    command: String,

    /// The arguments used to run the child process.
    command_args: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(io::Error),

    #[error("error file present at {0}")]
    ErrFilePresent(PathBuf),

    #[error("failed to spawn the child process: {0}")]
    NotSpawned(io::Error),

    #[error("failed to wait the child process: {0}")]
    WaitFailed(io::Error),

    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}

#[tracing::instrument(
    skip(opts),
    fields(
        command = %opts.command,
    )
)]
pub async fn run(opts: &Options) -> Result<(), Error> {
    // #[cfg(target_os = "linux")]
    // unsafe {
    //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    // }

    wait(opts).await?;

    let timer = timer(opts);

    let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
    let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;

    let mut child = spawn(opts).await?;
    let id = child.id().expect("fetching the OS-assigned process id") as libc::c_int;

    tokio::select! {
        biased;
        _ = future::select(
            interrupt.recv().boxed_local(),
            terminate.recv().boxed_local(),
        ) => {
            tracing::trace!(id, branch = "signaled");
        }
        _ = timer => {
            tracing::trace!(id, branch = "timedout");
        }
        wait_result = child.wait() => match wait_result {
            Ok(exit_status) => tracing::trace!(id, branch = "wait_exited", ?exit_status),
            Err(io_err) => tracing::warn!(id, branch = "wait_failed", %io_err),
        }
    }

    killpg(id).await;

    // TODO: Wait all descendant processes, if any.
    // Currently, the direct child is the only process to be waited before exiting.
    let result = match child.try_wait() {
        // It is possible for the child process to complete and exceed the timeout
        // without returning an error.
        Ok(Some(status)) => into_process_result(status),

        // The exit status is not available at this time.
        // The child process(es) may still be running.
        Ok(None) => match child.wait().await {
            Ok(status) => into_process_result(status),
            Err(err) => Err(WaitFailed(err)),
        },

        // Some error happens on collecting the child status.
        Err(err) => Err(WaitFailed(err)),
    };

    post(opts, result).await
}

#[tracing::instrument(skip(opts))]
async fn wait(opts: &Options) -> Result<(), Error> {
    let wait_files = opts.wait_files.iter().map(|ok_file| async move {
        let err_file = ok_file.with_extension("err");

        loop {
            tracing::trace!(wait_for = %ok_file.display());

            if ok_file.try_exists().map_err(Error::Io)? {
                return Ok(());
            }

            if err_file.try_exists().map_err(Error::Io)? {
                return Err(Error::ErrFilePresent(err_file));
            }

            time::sleep(Duration::from_millis(500)).await;
        }
    });

    future::try_join_all(wait_files).map_ok(|_| ()).await
}

#[tracing::instrument(skip(opts, result))]
async fn post(opts: &Options, result: Result<(), Error>) -> Result<(), Error> {
    let Some(path) = opts.post_file.as_ref() else {
        return Ok(());
    };

    fsutil::ensure_path_is_writable(path).await.map_err(Error::Io)?;

    if result.is_ok() {
        tracing::trace!(post_file = %path.display());
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    } else {
        let path = path.with_extension("err");
        tracing::trace!(post_file = %path.display());
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    }

    result
}

async fn spawn(opts: &Options) -> Result<Child, Error> {
    let mut cmd = StdCommand::new(opts.command.as_str());

    cmd.args(&opts.command_args);

    cmd.stdout(if let Some(path) = opts.stdout.as_ref() {
        fsutil::stdio_from(path, false).map_err(Error::Io).await?
    } else {
        Stdio::inherit()
    });

    cmd.stderr(if let Some(path) = opts.stderr.as_ref() {
        fsutil::stdio_from(path, false).map_err(Error::Io).await?
    } else {
        Stdio::inherit()
    });

    cmd.env_clear();

    // Put the child into a new process group.
    cmd.process_group(0);

    Command::from(cmd).spawn().map_err(NotSpawned)
}

fn timer(opts: &Options) -> future::Either<future::Pending<()>, time::Sleep> {
    match opts.timeout.as_ref() {
        None => future::pending().left_future(),
        Some(&dur) => time::sleep(dur.into()).right_future(),
    }
}

fn into_process_result(status: ExitStatus) -> Result<(), Error> {
    if status.success() {
        Ok(())
    } else if let Some(code) = status.code() {
        Err(ExitedUnsuccessfully(code))
    } else {
        // because `status.code()` returns `None`
        Err(KilledBySignal(status.signal().expect("WIFSIGNALED is true")))
    }
}

/// Kill the whole process group, in a graceful manner, to ensure there are no children left behind.
/// This mostly works, but not perfect because:
/// * it is easy to "escape" from the group.
/// * the PID is potentially reused at some point.
#[tracing::instrument]
async fn killpg(id: i32) {
    let killpg = |signal| unsafe {
        if libc::killpg(id, 0) == 0 {
            let killed = libc::killpg(id, signal);
            tracing::trace!(
                signal,
                killed,
                errno = io::Error::last_os_error().raw_os_error().unwrap_or(0)
            );
        }
    };

    killpg(libc::SIGTERM);
    // The time window between the `wait` returning and `SIGKILL` should be small.
    // Don't sleep too much.
    let delay = Duration::from_millis(100);
    time::sleep(delay).await;
    killpg(libc::SIGKILL);
}
