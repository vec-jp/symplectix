use std::env;
use std::ffi::OsString;
use std::io;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::ptr;
use std::sync::Arc;
use std::time::Duration;

use futures::{future, prelude::*};
use tokio::process::{Child, Command as TokioCommand};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;
use tracing::trace;

use crate::{KillCause, WaitStatus};

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
pub(crate) struct RunProc {
    /// Redirect the child process stdout.
    #[arg(long, value_name = "PATH")]
    stdout: Option<PathBuf>,

    /// Redirect the child process stderr.
    #[arg(long, value_name = "PATH")]
    stderr: Option<PathBuf>,

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    #[command(flatten)]
    pub(crate) timeout: Timeout,

    /// Check existence of given files before spawning the child process
    ///
    /// Note that the timeout duration does not elapse until the child is spawned.
    /// So the operations before spawning, i.e., waiting for files, never times out.
    #[arg(long = "wait", value_name = "PATH")]
    wait_on: Vec<PathBuf>,

    /// Create an empty file after the child process exits.
    #[arg(long, value_name = "PATH")]
    pub(crate) on_exit: Option<PathBuf>,

    /// The entrypoint of the child process.
    #[arg()]
    program: OsString, // ENTRYPOINT

    /// The arguments passed to the command.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<OsString>, // CMD
}

impl From<RunProc> for super::Command {
    fn from(data: RunProc) -> Self {
        super::Command { runp: Arc::new(data) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::Parser)]
pub(crate) struct Timeout {
    /// Kill the spawned process if it still running after the specified duration.
    #[arg(
        long = "timeout.duration",
        value_name = "DURATION",
        value_parser = humantime::parse_duration,
    )]
    pub(crate) duration: Option<Duration>,

    /// Exit with a zero status on timeout
    ///
    /// By default, timeout is considered as a failure,
    /// but it depends on the use cases whether what kind of status is success or not.
    // For example, timeout is not a failure for '//fuzzing:fuzz_test'.
    #[arg(long = "timeout.is-not-failure")]
    pub(crate) is_not_failure: bool,
}

#[derive(Debug)]
pub(crate) struct Process {
    child: Child,
    child_id: u32,
    runp: Arc<RunProc>,
}

impl From<Process> for super::Process {
    fn from(proc: Process) -> Self {
        super::Process { proc }
    }
}

impl RunProc {
    pub(super) async fn spawner(&self) -> io::Result<Spawner> {
        let mut cmd = std::process::Command::new(&self.program);

        cmd.args(&self.args[..]);

        cmd.stdout(if let Some(path) = self.stdout.as_ref() {
            fsutil::stdio_from(path, false).await?
        } else {
            Stdio::inherit()
        });

        cmd.stderr(if let Some(path) = self.stderr.as_ref() {
            fsutil::stdio_from(path, false).await?
        } else {
            Stdio::inherit()
        });

        cmd.env_clear().envs(env::vars().filter(|(key, _)| self.envs.contains(key)));

        // Put the child into a new process group.
        // A process group ID of 0 will use the process ID as the PGID.
        cmd.process_group(0);

        Ok(Spawner { spawnable: cmd })
    }
}

#[tracing::instrument]
async fn wait_on(paths: &[PathBuf]) -> Result<(), SpawnError> {
    let wait_files = paths.iter().map(|ok_file| async move {
        let err_file = ok_file.with_extension("err");

        loop {
            tracing::trace!(wait_for = %ok_file.display());

            if err_file.try_exists().map_err(SpawnError::Io)? {
                return Err(SpawnError::FoundErrFile(err_file));
            }

            if ok_file.try_exists().map_err(SpawnError::Io)? {
                return Ok(());
            }

            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    future::try_join_all(wait_files).map_ok(|_| ()).await
}

#[derive(Debug)]
pub(crate) struct Spawner {
    spawnable: std::process::Command,
}

#[derive(Debug)]
enum SpawnError {
    Io(io::Error),
    FoundErrFile(PathBuf),
}

impl Spawner {
    pub(crate) async fn spawn(self, cmd: Arc<RunProc>) -> io::Result<Process> {
        wait_on(&cmd.wait_on).await.map_err(|err| match err {
            SpawnError::Io(io_err) => io_err,
            SpawnError::FoundErrFile(path) => io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Found an error file at {}", path.display()),
            ),
        })?;

        let child = TokioCommand::from(self.spawnable).kill_on_drop(false).spawn()?;
        let child_id = child.id().expect("fetching child pid before polling should not fail");

        Ok(Process { child, child_id, runp: cmd })
    }
}

impl Process {
    #[tracing::instrument(skip(self))]
    pub(crate) async fn wait(mut self) -> io::Result<WaitStatus> {
        let mut sigint = signal(SignalKind::interrupt())?;
        let mut sigterm = signal(SignalKind::terminate())?;

        // TODO: Listen SIGCHLD
        // let mut sigchld = signal(SignalKind::child())?;

        use KillCause::*;

        let result = tokio::select! {
            biased;
            _ = sigint.recv() => {
                let sig = libc::SIGINT;
                self.kill(Signaled(sig)).gracefully().wait().await
            },
            _ = sigterm.recv() => {
                let sig = libc::SIGTERM;
                self.kill(Signaled(sig)).gracefully().wait().await
            },
            r = self.wait_child() => match r {
                Err(err) => {
                    self.kill(IoError(err.kind())).gracefully().wait().await
                }
                Ok(None) => {
                    self.kill(Timedout).gracefully().wait().await
                }
                Ok(Some(exit_status)) => {
                    Ok(WaitStatus{
                        exit_status,
                        kill_cause: exit_status.signal().map(Signaled),
                        runp: Arc::clone(&self.runp),
                    })
                }
            },
        };

        // If processes except the spawned one exists, reap them all here.
        unsafe {
            self.reap();
        }

        on_exit(self.runp.on_exit.as_ref(), result).await
    }

    /// Waits until the process exits or times out.
    /// For the case of timeout, Ok(None) will be returned.
    async fn wait_child(&mut self) -> io::Result<Option<ExitStatus>> {
        match self.runp.timeout.duration {
            // Always some because no timeout given.
            None => self.child.wait().await.map(Some),
            Some(dur) => match time::timeout(dur, self.child.wait()).await {
                Err(_elapsed) => Ok(None),
                Ok(status) => status.map(Some),
            },
        }
    }

    fn kill<T: Into<KillCause>>(&mut self, cause: T) -> Kill<'_> {
        Kill { proc: &mut *self, kill_cause: cause.into(), gracefully: false }
    }

    /// Waits all descendant processes to ensure there are no children left behind.
    unsafe fn reap(&self) {
        _ = killpg(self.child_id as libc::c_int, libc::SIGKILL);

        loop {
            // The WNOHANG option is used to indicate that the call should not block
            // if there are no processes that wish to report status.
            let pid = libc::waitpid(-1, ptr::null_mut(), libc::WNOHANG);
            // If this Error was constructed via last_os_error or from_raw_os_error,
            // then this function will return Some, otherwise it will return None.
            let err = io::Error::last_os_error().raw_os_error().unwrap();

            match (pid, err) {
                // If there are no children not previously awaited, with errno set to ECHILD.
                (-1, libc::ECHILD) => {
                    // No more children, we are done.
                    trace!(pid, err, "no children to be reaped");
                    return;
                }

                (_, libc::EINTR) => {
                    // This likely can't happen since we are calling libc::waitpid with WNOHANG.
                    trace!(pid, err, "got interrupted, try again");
                    continue;
                }

                // The pid is 0 when WNOHANG is specified and there are no stopped or exited children.
                // Otherwise, the process ID of the child represents a stopped or terminated child process.
                (pid, err) => {
                    trace!(pid, err, "continue reaping");
                    continue;
                }
            }
        }
    }
}

struct Kill<'a> {
    proc: &'a mut Process,
    kill_cause: KillCause,
    gracefully: bool,
}

impl<'a> Kill<'a> {
    fn gracefully(&mut self) -> &mut Self {
        self.gracefully = true;
        self
    }

    fn signal(&self) -> libc::c_int {
        match self.kill_cause {
            KillCause::Signaled(signal) => signal,
            _ => libc::SIGTERM,
        }
    }

    async fn wait(&mut self) -> io::Result<WaitStatus> {
        let kill_cause = Some(self.kill_cause.clone());

        // Notify the spawned process to be terminated.
        unsafe {
            // TODO: kill once, then fetch status for more detailed termination
            if self.gracefully {
                kill(self.proc.child_id as libc::c_int, self.signal())?;
                time::sleep(Duration::from_millis(1000)).await;

                if let Some(exit_status) = self.proc.child.try_wait()? {
                    let raw = exit_status.into_raw();
                    dbg!(
                        "gracefully",
                        &kill_cause,
                        exit_status.code(),
                        exit_status.signal(),
                        libc::WIFEXITED(raw),
                        libc::WEXITSTATUS(raw),
                        libc::WIFSIGNALED(raw),
                        libc::WTERMSIG(raw)
                    );
                    return Ok(WaitStatus {
                        exit_status,
                        kill_cause,
                        runp: Arc::clone(&self.proc.runp),
                    });
                }
                // The exit status is not available at this time. The child may still be running.
            }

            kill(self.proc.child_id as libc::c_int, libc::SIGKILL)?;
        }

        let exit_status = loop {
            match self.proc.child.try_wait()? {
                // The exit status is not available at this time. The child may still be running.
                // SIGKILL is sent just before entering the loop, but this happens because
                // kill(2) just sends the signal to the given process(es).
                // Once kill(2) returns, there is no guarantee that the signal has been delivered and handled.
                None => continue,
                Some(status) => break status,
            }
        };

        let raw = exit_status.into_raw();
        dbg!(
            "not gracefully",
            &kill_cause,
            exit_status.code(),
            exit_status.signal(),
            libc::WIFEXITED(raw),
            libc::WEXITSTATUS(raw),
            libc::WIFSIGNALED(raw),
            libc::WTERMSIG(raw)
        );

        Ok(WaitStatus { exit_status, kill_cause, runp: Arc::clone(&self.proc.runp) })
    }
}

unsafe fn kill(pid: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
    if libc::kill(pid, signal) == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

unsafe fn killpg(pgrp: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
    if libc::killpg(pgrp, signal) == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[tracing::instrument]
async fn on_exit(path: Option<&PathBuf>, result: io::Result<WaitStatus>) -> io::Result<WaitStatus> {
    if let Some(path) = path {
        if matches!(result, Ok(ref status) if status.success()) {
            fsutil::create_file(path, true).await?;
        } else {
            fsutil::create_file(path.with_extension("err"), true).await?;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use testing::TempDirExt;
    use tokio::task;

    use super::{wait_on, RunProc, SpawnError, Timeout};
    use crate::Process;

    impl RunProc {
        pub(super) fn argv<I, T>(argv: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: Into<OsString> + Clone,
        {
            let mut argv = argv.into_iter();

            RunProc {
                stdout: None,
                stderr: None,
                envs: vec![],
                timeout: Timeout { duration: None, is_not_failure: false },
                wait_on: vec![],
                on_exit: None,
                program: argv.next().unwrap().into(),
                args: argv.map(|s| s.into()).collect::<Vec<_>>(),
            }
        }

        pub(super) fn timeout(mut self, duration: Duration) -> Self {
            self.timeout = Timeout { duration: Some(duration), is_not_failure: false };
            self
        }
    }

    async fn spawn<R: Into<crate::Command>>(runp: R) -> io::Result<Process> {
        let r: crate::Command = runp.into();
        r.spawn().await
    }

    #[tokio::test]
    async fn spawn_commands() {
        // Should be ok because this checks whether we can spawn the process.
        assert!(spawn(RunProc::argv(["test", "-e", "/xxx"])).await.is_ok());
        assert!(spawn(RunProc::argv(["test", "-e", "/yyy"])).await.is_ok());
        assert!(spawn(RunProc::argv(["not_command", "foo"])).await.is_err());
    }

    #[tokio::test]
    async fn run_process() {
        let sleep = spawn(RunProc::argv(["sleep", "0.1"])).await.unwrap();
        let Ok(status) = sleep.wait().await else {
            panic!("failed to fetch wait status");
        };
        assert!(status.success(), "expected that the command 'sleep' exit successfully");

        let sleep =
            spawn(RunProc::argv(["sleep", "1"]).timeout(Duration::from_millis(100))).await.unwrap();
        let Ok(status) = sleep.wait().await else {
            panic!("failed to fetch wait status");
        };
        assert!(status.timedout(), "expected that the command 'sleep' times out");
    }

    fn create_files<P, T>(temp_dir: &testing::TempDir, paths: T) -> Vec<PathBuf>
    where
        P: AsRef<Path>,
        T: AsRef<[P]>,
    {
        paths
            .as_ref()
            .iter()
            .map(|path| {
                let (_file, path) = temp_dir
                    .create_file(fs::OpenOptions::new().create(true).read(true).write(true), path)
                    .expect("create a temporary file");
                path
            })
            .collect()
    }

    #[tokio::test]
    async fn wait_for_files() {
        let temp_dir = testing::tempdir();

        wait_on(&[]).await.expect("wait for nothing");

        let mut oks = create_files(&temp_dir, vec!["東/新宿/ok", "柏/の/葉/ok", "秋/葉/原/ok"]);
        wait_on(&oks).await.expect("waiting for files created just before");

        let err = create_files(&temp_dir, vec!["0.err"]);
        wait_on(&oks).await.expect("affected by an error file not waiting for");

        let more_oks = create_files(&temp_dir, vec!["0"]);
        oks.extend_from_slice(&more_oks);

        match wait_on(&oks)
            .await
            .expect_err("should be an error if '0' and '0.err' exist at the same time")
        {
            SpawnError::FoundErrFile(p) => {
                assert_eq!(p, err[0]);
            }
            others => {
                panic!("unexpected error: {others:?}")
            }
        }

        fs::remove_file(&more_oks[0]).unwrap();
        match wait_on(&oks)
            .await
            .expect_err("should be an error because the error file '0.err' present")
        {
            SpawnError::FoundErrFile(p) => {
                assert_eq!(p, err[0]);
            }
            others => {
                panic!("unexpected error: {others:?}")
            }
        }

        fs::remove_file(&err[0]).unwrap();
        // `wait` does not finish until the file "0" is created.
        let h = task::spawn(async move { wait_on(&oks).await });
        create_files(&temp_dir, vec!["0"]);
        h.await.unwrap().expect("should be ok");

        temp_dir.close().unwrap();
    }
}
