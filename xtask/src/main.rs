use std::{collections::HashMap, process::Command};

use cargo_metadata::{camino::Utf8PathBuf, Metadata, MetadataCommand, Package};
use clap::Parser;
use cli_xtask::fs::ToRelative;

mod build;
mod exec;
mod lint;
mod test;

#[derive(Debug, Parser)]
enum Args {
    /// Execute commands on this workspace and all sub workspaces
    Exec(exec::Args),
    /// Execute all lint commands on this workspace and all sub workspaces
    Lint(lint::Args),
    /// Execute cargo build on this workspace and all sub workspaces
    Build(build::Args),
    /// Execute cargo test on this workspace and all sub workspaces
    Test(test::Args),
}

impl Args {
    fn run(&self) -> eyre::Result<()> {
        match self {
            Self::Exec(args) => args.run(),
            Self::Lint(args) => args.run(),
            Self::Build(args) => args.run(),
            Self::Test(args) => args.run(),
        }
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_target(false)
        .try_init()
        .map_err(|e| eyre::eyre!(e))?;

    tracing::info!("Running on {}", std::env::current_dir()?.display());
    let _workspaces = all_workspaces()?;
    Args::parse().run()?;

    Ok(())
}

fn all_workspaces() -> eyre::Result<Vec<(Utf8PathBuf, Metadata)>> {
    let mut workspaces = HashMap::new();
    for entry in glob::glob("**/Cargo.toml")? {
        let path = Utf8PathBuf::try_from(entry?)?;
        let path = path.to_relative();
        tracing::debug!("Found manifest {}", path);

        let metadata = MetadataCommand::new().manifest_path(path).exec()?;
        workspaces
            .entry(metadata.workspace_root.clone())
            .or_insert_with(|| {
                tracing::debug!("Found workspace {}", metadata.workspace_root.to_relative());
                metadata
            });
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
    tracing::info!("Executing command on {}", workspace_root.to_relative());
    tracing::info!("  $ {} {}", command, args.join(" "));
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
