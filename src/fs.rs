use std::{fmt, fs::File};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use eyre::ensure;

/// Create a new [`File`](std::fs::File) from a `path`, and output the path to log.
#[tracing::instrument(name = "create_file" fields(path = %path.as_ref().to_relative()), err)]
pub fn create_file(path: impl AsRef<Utf8Path>) -> eyre::Result<File> {
    tracing::info!("creating file");
    let path = path.as_ref();
    let file = File::create(path)?;
    Ok(file)
}

/// Create a new directory if it doesn't exist, or remove all its contents if exists.
pub fn create_or_cleanup_dir(dir: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    Ok(())
}

/// Copy a file from `from` to `to`, and output those path to log.
#[tracing::instrument(name = "copy" skip_all, err)]
pub fn copy(from: impl AsRef<Utf8Path>, to: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    tracing::info!("{} -> {}", from.to_relative(), to.to_relative());
    ensure!(from.is_file(), "not a file: {}", from.to_relative());
    std::fs::copy(from, to)?;
    Ok(())
}

/// Convert a path to a path relative to the root of the cargo workspace which implements [`Display`](std::fmt::Display).
pub trait ToRelative {
    /// Tye type of the converted path that implements [`Display`](std::fmt::Display).
    type Output: fmt::Display;

    /// Convert the path to a path relative to the root of the cargo workspace which implements [`Display`](std::fmt::Display).
    fn to_relative(self) -> Self::Output;
}

impl<'a> ToRelative for &'a Utf8Path {
    type Output = &'a Utf8Path;
    fn to_relative(self) -> Self::Output {
        let relative = self
            .strip_prefix(&crate::get_metadata().workspace_root)
            .unwrap_or(self);
        if relative == "" {
            Utf8Path::new(".")
        } else {
            relative
        }
    }
}

impl<'a> ToRelative for &'a Utf8PathBuf {
    type Output = &'a Utf8Path;
    fn to_relative(self) -> Self::Output {
        <&Utf8Path>::to_relative(self)
    }
}
