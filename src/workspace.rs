use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, MetadataCommand, Package,
};
use once_cell::sync::Lazy;
use walkdir::WalkDir;

use crate::fs::ToRelative;

static WORKSPACES: Lazy<Vec<Metadata>> = Lazy::new(|| {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = Utf8PathBuf::try_from(current_dir).unwrap();
    collect_workspaces(&current_dir).unwrap()
});

/// Returns a current cargo workspace metadata.
pub fn current() -> &'static Metadata {
    &WORKSPACES[0]
}

/// Returns metadata for all cargo workspaces under the current workspace.
pub fn all() -> &'static [Metadata] {
    &WORKSPACES
}

fn collect_workspaces(base_dir: &Utf8Path) -> eyre::Result<Vec<Metadata>> {
    let mut workspaces = HashMap::new();
    let mut target_dirs = HashSet::new();

    let current_workspace = MetadataCommand::new().current_dir(base_dir).exec()?;
    let current_workspace_root = &current_workspace.workspace_root;

    let mut it = WalkDir::new(&current_workspace_root)
        .sort_by(
            // Sort files before directories.
            // This is to make sure that `target_dirs` is updated before files in it are iterated.
            |a, b| match (a.file_type().is_file(), b.file_type().is_file()) {
                (true, true) => a.file_name().cmp(b.file_name()),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (false, false) => a.file_name().cmp(b.file_name()),
            },
        )
        .into_iter();

    while let Some(entry) = it.next() {
        let entry = entry?;
        let path = <&Utf8Path>::try_from(entry.path())?;

        // Check if the path is a cargo manifest file.
        if entry.file_type().is_file() && path.file_name() == Some("Cargo.toml") {
            tracing::debug!("Found manifest {}", path.to_relative());
            let metadata = MetadataCommand::new().manifest_path(path).exec()?;
            match workspaces.entry(metadata.workspace_root.clone()) {
                Entry::Occupied(_e) => {}
                Entry::Vacant(e) => {
                    if metadata.target_directory.is_dir() {
                        let target_dir = metadata.target_directory.canonicalize_utf8()?;
                        tracing::debug!(
                            "Found workspace {}",
                            metadata.workspace_root.to_relative()
                        );
                        target_dirs.insert(target_dir);
                    }
                    e.insert(metadata);
                }
            }
        }

        // Skip the .git directory.
        if entry.file_type().is_dir() && path.file_name() == Some(".git") {
            tracing::debug!("Skipping git directory {}", path.to_relative());
            it.skip_current_dir();
            continue;
        }

        // Skip the current workspace's target directories.
        // This prevents the `target/package` directory from being included in the workspace.
        if entry.file_type().is_dir() && target_dirs.contains(&path.canonicalize_utf8()?) {
            tracing::debug!("Skipping target directory {}", path.to_relative());
            it.skip_current_dir();
            continue;
        }
    }

    // Sort workspaces by their root directory.
    // The shallowest workspace should come first.
    let mut workspaces = workspaces
        .into_iter()
        .map(|(_path, ws)| ws)
        .collect::<Vec<_>>();
    workspaces.sort_by(|a, b| a.workspace_root.cmp(&b.workspace_root));

    Ok(workspaces)
}

/// Returns a list of all feature combinations for the given package.
pub fn feature_combination_args(package: &Package) -> Vec<Vec<&str>> {
    if package.features.is_empty() {
        return vec![vec![]];
    }

    let features = package.features.keys();
    let mut args = vec![vec!["--all-features"], vec!["--no-default-features"]];
    for feature in features {
        args.push(vec!["--features", feature, "--no-default-features"]);
    }
    args
}
