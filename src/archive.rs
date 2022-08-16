use cargo_metadata::camino::Utf8Path;
use flate2::{write::GzEncoder, Compression};

use crate::fs::ToRelative;

/// Create a `.tar.gz` archive from a directory.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> eyre::Result<()> {
/// cli_xtask::archive::create("foo.tar.gz", ["./foo/", "./bar.txt"])?;
/// # Ok(())
/// # }
/// ```
#[tracing::instrument(name = "archive::create", skip_all, err)]
pub fn create(
    archive_path: impl AsRef<Utf8Path>,
    src: impl IntoIterator<Item = impl AsRef<Utf8Path>>,
) -> eyre::Result<()> {
    let archive = crate::fs::create_file(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for src in src.into_iter() {
        let src = src.as_ref();
        if src.is_file() {
            tracing::info!("adding file: {}", src.to_relative());
            tar.append_path_with_name(src, src.file_name().unwrap())?;
        } else {
            for entry in src.read_dir_utf8()? {
                let entry = entry?;
                let artifact_name = entry.file_name();
                if entry.metadata()?.is_file() {
                    tracing::info!("adding file: {}", entry.path().to_relative());
                    tar.append_path_with_name(entry.path(), artifact_name)?;
                } else {
                    tracing::info!("adding directory: {}", entry.path().to_relative());
                    tar.append_dir_all(artifact_name, entry.path())?;
                }
            }
        }
    }

    Ok(())
}
