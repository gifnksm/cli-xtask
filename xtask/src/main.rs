use cargo_metadata::Package;
use clap::Parser;
use cli_xtask::{Config, ConfigBuilder};

mod exec;
mod lint;

#[derive(Debug, Parser)]
enum Args {
    #[clap(flatten)]
    Command(cli_xtask::Command),
    /// Run commands on all workspaces in the current directory and subdirectories
    Exec(exec::Args),
    /// Run all lint commands on all workspaces in the current directory and subdirectories
    Lint(lint::Args),
}

impl Args {
    fn run(&self, config: &Config) -> eyre::Result<()> {
        match self {
            Self::Command(args) => args.run(config),
            Self::Exec(args) => args.run(),
            Self::Lint(args) => args.run(config),
        }
    }
}

fn main() -> eyre::Result<()> {
    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger()?;

    tracing::info!("Running on {}", std::env::current_dir()?.display());
    let config = ConfigBuilder::new().build();
    Args::parse().run(&config)?;

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
