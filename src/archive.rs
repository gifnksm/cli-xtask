//! Utilities for creating archives.

use cargo_metadata::camino::Utf8Path;
use flate2::{write::GzEncoder, Compression};

use crate::{fs::ToRelative, Result};

/// Create a `tar.gz` archive from the given paths.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> cli_xtask::Result<()> {
/// cli_xtask::archive::create("foo.tar.gz", ["./foo/", "./bar.txt"])?;
/// # Ok(())
/// # }
/// ```
#[tracing::instrument(name = "archive::create", skip_all, err)]
pub fn create(
    archive_path: impl AsRef<Utf8Path>,
    src: impl IntoIterator<Item = impl AsRef<Utf8Path>>,
) -> Result<()> {
    let archive = crate::fs::create_file(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for src in src.into_iter() {
        let src = src.as_ref();
        if src.is_file() {
            tracing::info!("adding file: {}", src.to_relative());
            tar.append_path_with_name(src, src.file_name().unwrap())?;
        } else {
            tracing::info!("adding directory: {}", src.to_relative());
            tar.append_dir_all(src.file_name().unwrap(), src)?;
        }
    }

    // errors in drop are ignored, so we should flush the data here
    let enc = tar.into_inner()?;
    let archive = enc.finish()?;
    archive.sync_all()?;

    Ok(())
}
