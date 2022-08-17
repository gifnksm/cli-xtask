use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
    process::Command,
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, MetadataCommand, Package,
};
use clap::Parser;
use cli_xtask::fs::ToRelative;
use walkdir::WalkDir;

mod build;
mod clippy;
mod exec;
mod fmt;
mod lint;
mod rdme;
mod test;
mod udeps;

#[derive(Debug, Parser)]
enum Args {
    /// Run `cargo build` on all workspaces in the current directory and subdirectories
    Build(build::Args),
    /// Run `cargo clippy` on all workspaces in the current directory and subdirectories
    Clippy(clippy::Args),
    /// Run commands on all workspaces in the current directory and subdirectories
    Exec(exec::Args),
    /// Run `cargo fmt` on all workspaces in the current directory and subdirectories
    Fmt(fmt::Args),
    /// Run all lint commands on all workspaces in the current directory and subdirectories
    Lint(lint::Args),
    /// Run `cargo rdme` on all workspaces in the current directory and subdirectories
    Rdme(rdme::Args),
    /// Run `cargo test` on all workspaces in the current directory and subdirectories
    Test(test::Args),
    /// Run `cargo udeps` on all workspaces in the current directory and subdirectories
    Udeps(udeps::Args),
}

impl Args {
    fn run(&self) -> eyre::Result<()> {
        match self {
            Self::Build(args) => args.run(),
            Self::Clippy(args) => args.run(),
            Self::Exec(args) => args.run(),
            Self::Fmt(args) => args.run(),
            Self::Lint(args) => args.run(),
            Self::Rdme(args) => args.run(),
            Self::Test(args) => args.run(),
            Self::Udeps(args) => args.run(),
        }
    }
}

fn main() -> eyre::Result<()> {
    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger()?;

    tracing::info!("Running on {}", std::env::current_dir()?.display());
    Args::parse().run()?;

    Ok(())
}

fn all_workspaces() -> eyre::Result<Vec<(Utf8PathBuf, Metadata)>> {
    let mut workspaces = HashMap::new();
    let mut target_dirs = HashSet::new();

    let current_workspace_root = &cli_xtask::current_workspace().workspace_root;

    let mut it = WalkDir::new(&current_workspace_root)
        .sort_by(
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

        if entry.file_type().is_file() && path.file_name() == Some("Cargo.toml") {
            tracing::debug!("Found manifest {path}");
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

        if entry.file_type().is_dir() && path.file_name() == Some(".git") {
            tracing::debug!("Skipping git directory {}", path.to_relative());
            it.skip_current_dir();
            continue;
        }
        if entry.file_type().is_dir() && target_dirs.contains(&path.canonicalize_utf8()?) {
            tracing::debug!("Skipping target directory {}", path.to_relative());
            it.skip_current_dir();
            continue;
        }
    }

    let mut workspaces = workspaces.into_iter().collect::<Vec<_>>();
    workspaces.sort_by(|(a, _), (b, _)| a.cmp(b));
    Ok(workspaces)
}

fn execute_on(
    metadata: &Metadata,
    command: impl AsRef<str>,
    args: impl IntoIterator<Item = impl Into<String>>,
) -> eyre::Result<()> {
    let command = command.as_ref();
    let args = args.into_iter().map(Into::into).collect::<Vec<_>>();

    let workspace_root = &metadata.workspace_root;
    tracing::info!(
        "[{}]$ {} {}",
        workspace_root.to_relative(),
        command,
        args.join(" ")
    );
    let status = Command::new(command)
        .args(args)
        .current_dir(workspace_root)
        .status()?;

    if !status.success() {
        tracing::error!(
            "Command for {} failed with status {}",
            workspace_root.to_relative(),
            status.code().unwrap()
        );
        return Err(eyre::eyre!(
            "command for {} failed with status {}",
            workspace_root.to_relative(),
            status.code().unwrap()
        ));
    }
    Ok(())
}

fn feature_combinations(package: &Package) -> Vec<Vec<&str>> {
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
