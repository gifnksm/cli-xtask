use cargo_metadata::{Metadata, MetadataCommand};
use clap::Parser;
use cli_xtask::fs::ToRelative;

mod exec;

#[derive(Debug, Parser)]
enum Args {
    /// Execute commands on this workspace and all sub workspaces
    Exec(exec::Args),
}

impl Args {
    fn run(&self) -> eyre::Result<()> {
        match self {
            Args::Exec(args) => args.run(),
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
    Args::parse().run()?;

    Ok(())
}

fn metadata(include_root: bool) -> eyre::Result<Vec<Metadata>> {
    let mut metadata = vec![];
    let root_metadata = MetadataCommand::new().exec()?;
    let workspace_root = root_metadata.workspace_root.clone();

    if include_root {
        tracing::info!("Loading metadata for {}", workspace_root.to_relative());
        metadata.push(root_metadata);
    }

    for entry in workspace_root.join("examples").read_dir_utf8()? {
        let entry = entry?;
        let manifest_path = entry.path().join("Cargo.toml");
        if manifest_path.is_file() {
            tracing::info!("Loading metadata for {}", entry.path().to_relative());
            let meta = MetadataCommand::new().manifest_path(manifest_path).exec()?;
            metadata.push(meta);
        }
    }

    Ok(metadata)
}
