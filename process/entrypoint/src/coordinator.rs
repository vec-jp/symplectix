use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::Parser;
use futures::future;
use futures::prelude::*;
use tokio::time;

use crate::fsutil;
use crate::ProcessWrapper;
use crate::{Error, Result};

#[derive(Debug, Clone, Parser)]
pub struct Coordinator {
    /// List of paths to wait for before spawning the child process.
    #[arg(long = "wait-file", value_name = "PATH")]
    pub wait_files: Vec<PathBuf>,

    /// Create a file after the child process exits successfully.
    #[arg(long, value_name = "PATH")]
    pub post_file: Option<PathBuf>,

    #[command(flatten)]
    pub entrypoint: ProcessWrapper,
}

impl Coordinator {
    #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> Result {
        wait(&self.wait_files).await?;
        let result = self.entrypoint.run().await;
        post(&self.post_file, result).await
    }
}

#[tracing::instrument]
async fn wait(wait_files: &[PathBuf]) -> Result {
    let wait_files = wait_files.iter().map(|ok_file| async move {
        let err_file = ok_file.with_extension("err");

        loop {
            tracing::trace!(wait_for = %ok_file.display());

            if err_file.try_exists().map_err(Error::Io)? {
                return Err(Error::ErrFileExists(err_file));
            }

            if ok_file.try_exists().map_err(Error::Io)? {
                return Ok(());
            }

            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    future::try_join_all(wait_files).map_ok(|_| ()).await
}

#[tracing::instrument(skip(post_file))]
async fn post<P: AsRef<Path>>(post_file: &Option<P>, result: Result) -> Result {
    let Some(path) = post_file.as_ref() else {
        return Ok(());
    };

    fsutil::ensure_path_is_writable(path).await.map_err(Error::Io)?;

    if result.is_ok() {
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    } else {
        let path = path.as_ref().with_extension("err");
        fsutil::create_file(path, true).await.map_err(Error::Io)?;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use testing::TempDirExt;
    use tokio::task;

    fn create_files<P, T>(temp_dir: &testing::TempDir, paths: T) -> (Vec<fs::File>, Vec<PathBuf>)
    where
        P: AsRef<Path>,
        T: AsRef<[P]>,
    {
        paths
            .as_ref()
            .iter()
            .map(|path| {
                temp_dir
                    .create_file(fs::OpenOptions::new().create(true).read(true).write(true), path)
                    .expect("create a temporary file")
            })
            .unzip()
    }

    #[tokio::test]
    async fn wait_for_files() {
        let temp_dir = testing::tempdir();

        wait(&[]).await.expect("wait for nothing");

        let (_, mut oks) = create_files(&temp_dir, vec!["柏/の/葉/ok", "秋/葉/原/ok"]);
        wait(&oks).await.expect("waiting for files created just before");

        let (_, errs) = create_files(&temp_dir, vec!["0.err"]);
        wait(&oks).await.expect("affected by an error file not waiting for");

        let (_, more_oks) = create_files(&temp_dir, vec!["0"]);
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
