use std::io;
use std::path::PathBuf;

mod command;

pub use command::{Command, Process};

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Process errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(io::Error),

    #[error("error file exists at {0}")]
    ErrFileExists(PathBuf),

    #[error("failed to spawn the child process: {0}")]
    NotSpawned(io::Error),

    #[error("failed to wait the child process: {0}")]
    WaitFailed(io::Error),

    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}
