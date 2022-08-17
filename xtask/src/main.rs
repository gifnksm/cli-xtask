use clap::Parser;
use cli_xtask::{Config, ConfigBuilder};

mod exec;

#[derive(Debug, Parser)]
enum Args {
    #[clap(flatten)]
    Command(cli_xtask::Command),
    /// Run commands on all workspaces in the current directory and subdirectories
    Exec(exec::Args),
}

impl Args {
    fn run(&self, config: &Config) -> eyre::Result<()> {
        match self {
            Self::Command(args) => args.run(config),
            Self::Exec(args) => args.run(),
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
