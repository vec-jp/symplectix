use std::ffi::OsString;
use std::io;
use std::process::{ExitCode, ExitStatus};
use std::sync::Arc;

mod imp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    runp: Arc<imp::RunProc>,
}

#[derive(Debug)]
pub struct Process {
    proc: imp::Process,
}

impl Command {
    pub fn from_args_os<I, T>(args_os: I) -> Command
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Command { runp: Arc::new(<imp::RunProc as clap::Parser>::parse_from(args_os)) }
    }

    pub async fn spawn(&self) -> io::Result<Process> {
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
        // }

        let spawner = self.runp.spawner().await?;
        let spawned = spawner.spawn(Arc::clone(&self.runp)).await?;
        Ok(Process::from(spawned))
    }
}

impl Process {
    #[tracing::instrument(skip(self))]
    pub async fn wait(self) -> io::Result<WaitStatus> {
        self.proc.wait().await
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitStatus {
    exit_status: ExitStatus,
    kill_cause: Option<KillCause>,
    runp: Arc<imp::RunProc>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
enum KillCause {
    #[error("Killed the process because got an error on waiting for the child ({0})")]
    IoError(io::ErrorKind),
    #[error("Killed the process because got a signal ({0})")]
    Signaled(libc::c_int),
    #[error("Killed the process because it has timed out")]
    Timedout,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ProcStatusError {
    imp: ProcStatusErrorImp,
}

impl ProcStatusError {
    pub fn exit_code(&self) -> ExitCode {
        match self.imp {
            ProcStatusErrorImp::Killed { kill_cause: KillCause::Timedout, .. } => {
                ExitCode::from(124)
            }
            ProcStatusErrorImp::Killed { kill_cause: KillCause::IoError(_), .. } => {
                ExitCode::from(125)
            }
            ProcStatusErrorImp::Killed { kill_cause: KillCause::Signaled(s), .. } => {
                ExitCode::from(128 + s as u8)
            }
            ProcStatusErrorImp::Failed { exit_status } => {
                exit_status.code().map(|c| ExitCode::from(c as u8)).unwrap_or(ExitCode::FAILURE)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ProcStatusErrorImp {
    #[error("{kill_cause}")]
    Killed { exit_status: ExitStatus, kill_cause: KillCause },

    #[error("The spawned process exited with {exit_status}")]
    Failed { exit_status: ExitStatus },
}

impl WaitStatus {
    pub fn success(&self) -> bool {
        self.exit_ok().is_ok()
    }

    // fn from_exit_status(exit_status: ExitStatus) -> WaitStatus {
    //     WaitStatus { exit_status, cause: None, timeout: self.timeout }
    // }

    pub fn exit_ok(&self) -> Result<(), ProcStatusError> {
        // Need more consideration:
        // * If the killed process exits with status 0, is it success or failure?
        match self {
            WaitStatus { exit_status, kill_cause: Some(kill_cause), runp: cfg } => {
                if matches!(kill_cause, KillCause::Timedout) && cfg.timeout.is_not_failure {
                    Ok(())
                } else {
                    // The spawned process has been killed by some reasons.
                    Err(ProcStatusError {
                        imp: ProcStatusErrorImp::Killed {
                            exit_status: *exit_status,
                            kill_cause: kill_cause.clone(),
                        },
                    })
                }
            }
            WaitStatus { exit_status, kill_cause: None, .. } => {
                if exit_status.success() {
                    Ok(())
                } else {
                    // Not killed, exited with a failure.
                    Err(ProcStatusError {
                        imp: ProcStatusErrorImp::Failed { exit_status: *exit_status },
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KillCause, WaitStatus};

    impl WaitStatus {
        pub(crate) fn timedout(&self) -> bool {
            matches!(self.kill_cause, Some(KillCause::Timedout))
        }
    }
}
