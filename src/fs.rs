use std::{borrow::Cow, fmt, fs::File};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use eyre::ensure;

/// Create a new [`File`](std::fs::File) from a `path`, and output the path to log.
#[tracing::instrument(name = "create_file" fields(path = %path.as_ref().to_relative()), err)]
pub fn create_file(path: impl AsRef<Utf8Path>) -> eyre::Result<File> {
    let path = path.as_ref();
    let dir = path
        .parent()
        .ok_or_else(|| eyre::eyre!("path has no parent"))?;
    create_dir(dir)?;

    tracing::info!("creating file");
    let file = File::create(path)?;
    Ok(file)
}

/// Create a new directory if it doesn't exist, and output the path to log.
#[tracing::instrument(name = "create_dir" fields(path = %path.as_ref().to_relative()), err)]
pub fn create_dir(path: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let path = path.as_ref();
    if !path.is_dir() {
        tracing::info!("creating directory");
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Remove a directory if exists and output the path to log.
#[tracing::instrument(name = "remove_dir" fields(path = %path.as_ref().to_relative()), err)]
pub fn remove_dir(path: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        tracing::info!("removing directory");
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Create a new directory if it doesn't exist, or remove all its contents if exists.
pub fn create_or_cleanup_dir(dir: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let dir = dir.as_ref();
    remove_dir(dir)?;
    create_dir(dir)?;
    Ok(())
}

/// Copy a file from `from` to `to`, and output those path to log.
#[tracing::instrument(name = "copy" skip_all, err)]
pub fn copy(from: impl AsRef<Utf8Path>, to: impl AsRef<Utf8Path>) -> eyre::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    if let Some(parent) = to.parent() {
        create_dir(parent)?;
    }
    tracing::info!("{} -> {}", from.to_relative(), to.to_relative());
    ensure!(from.is_file(), "not a file: {}", from.to_relative());
    std::fs::copy(from, to)?;
    Ok(())
}

/// Convert a path to a path relative to the current directory which implements [`Display`](std::fmt::Display).
pub trait ToRelative {
    /// Tye type of the converted path that implements [`Display`](std::fmt::Display).
    type Output: fmt::Display;

    /// Convert the path to a path relative to the current directory which implements [`Display`](std::fmt::Display).
    fn to_relative(self) -> Self::Output;
}

impl<'a> ToRelative for &'a Utf8Path {
    type Output = Cow<'a, Utf8Path>;
    fn to_relative(self) -> Self::Output {
        if self.is_relative() {
            return Cow::Borrowed(self);
        }

        let current_dir = std::env::current_dir()
            .ok()
            .and_then(|path| Utf8PathBuf::try_from(path).ok());
        let mut current_dir = current_dir.as_deref();
        let mut prefix = Utf8PathBuf::new();
        while let Some(cur_dir) = current_dir {
            if let Ok(relative) = self.strip_prefix(cur_dir) {
                if prefix != "" {
                    return Cow::Owned(prefix.join(relative));
                }
                return if relative == "" {
                    Cow::Borrowed(Utf8Path::new("."))
                } else {
                    Cow::Borrowed(relative)
                };
            }
            current_dir = cur_dir.parent();
            prefix.push("..");
        }

        Cow::Borrowed(self)
    }
}

impl<'a> ToRelative for &'a Utf8PathBuf {
    type Output = Cow<'a, Utf8Path>;
    fn to_relative(self) -> Self::Output {
        <&Utf8Path>::to_relative(self)
    }
}
