use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use futures::future;
use futures::prelude::*;
use tokio::time;

use entrypoint::Command;
use entrypoint::{Error, Result};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    Coordinator::parse().run().await
}

#[derive(Debug, Clone, Parser)]
pub struct Coordinator {
    /// List of paths to wait for before spawning the child process.
    ///
    /// The timeout clock does not tick until the child spawns.
    /// So the operations before spawning, i.e., while waiting for `wait-file`s, never times out.
    #[arg(long = "wait", value_name = "PATH")]
    wait_files: Vec<PathBuf>,

    /// Create a file after the child process exits successfully.
    #[arg(long = "post", value_name = "PATH")]
    post_file: Option<PathBuf>,

    #[command(flatten)]
    command: Command,
}

impl Coordinator {
    #[tracing::instrument(
        skip(self),
        fields(
            wait_files = ?self.wait_files,
            post_file = ?self.post_file,
        )
    )]
    pub async fn run(self) -> anyhow::Result<()> {
        wait(&self.wait_files).await.context("wait files")?;
        let result = self.command.run().await;
        post(&self.post_file, result).await.map_err(anyhow::Error::from)
    }
}

#[tracing::instrument]
async fn wait(wait_files: &[PathBuf]) -> anyhow::Result<()> {
    let wait_files = wait_files.iter().map(|ok_file| async move {
        let err_file = ok_file.with_extension("err");

        loop {
            tracing::trace!(wait_for = %ok_file.display());

            if err_file.try_exists().context("try_exists")? {
                anyhow::bail!("error files exists at {}", err_file.display());
            }

            if ok_file.try_exists().context("try_exists")? {
                return Ok(());
            }

            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    future::try_join_all(wait_files).map_ok(|_| ()).await
}

#[tracing::instrument]
async fn post(post_file: &Option<PathBuf>, result: Result) -> Result {
    let Some(path) = post_file.as_ref() else {
        return Ok(());
    };

    fsutil::ensure_path_is_writable(path).await.map_err(Error::Io)?;

    if result.is_ok() {
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    } else {
        let path = path.with_extension("err");
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::Path;
    use testing::TempDirExt;
    use tokio::task;

    fn create_files<P, T>(temp_dir: &testing::TempDir, paths: T) -> Vec<PathBuf>
    where
        P: AsRef<Path>,
        T: AsRef<[P]>,
    {
        let (files, paths): (Vec<fs::File>, Vec<PathBuf>) = paths
            .as_ref()
            .iter()
            .map(|path| {
                temp_dir
                    .create_file(fs::OpenOptions::new().create(true).read(true).write(true), path)
                    .expect("create a temporary file")
            })
            .unzip();

        drop(files);
        paths
    }

    #[tokio::test]
    async fn wait_for_files() {
        let temp_dir = testing::tempdir();

        wait(&[]).await.expect("wait for nothing");

        let mut oks = create_files(&temp_dir, vec!["柏/の/葉/ok", "秋/葉/原/ok"]);
        wait(&oks).await.expect("waiting for files created just before");

        let errs = create_files(&temp_dir, vec!["0.err"]);
        wait(&oks).await.expect("affected by an error file not waiting for");

        let more_oks = create_files(&temp_dir, vec!["0"]);
        oks.extend_from_slice(&more_oks);

        let err = wait(&oks)
            .await
            .expect_err("should be an error if '0' and '0.err' exist at the same time");
        match err {
            crate::Error::ErrFileExists(p) => {
                assert_eq!(p, errs[0]);
            }
            others => {
                panic!("unexpected error: {others:?}")
            }
        }

        fs::remove_file(&more_oks[0]).unwrap();
        let err = wait(&oks)
            .await
            .expect_err("should be an error because the error file '0.err' present");

        match err {
            crate::Error::ErrFileExists(p) => {
                assert_eq!(p, errs[0]);
            }
            others => {
                panic!("unexpected error: {others:?}")
            }
        }

        fs::remove_file(&errs[0]).unwrap();
        // `wait` does not finish until the file "0" is created.
        let h = task::spawn(async move { wait(&oks).await });
        create_files(&temp_dir, vec!["0"]);

        h.await.unwrap().expect("should be ok")
    }
}
