use std::io;

mod command;

pub use command::{Command, Process};

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Process errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(io::Error),

    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}
