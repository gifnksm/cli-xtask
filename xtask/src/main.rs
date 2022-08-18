use clap::{CommandFactory, Parser};
use cli_xtask::{
    args::Verbosity,
    config::{Config, ConfigBuilder},
};
use tracing::Level;

#[derive(Debug, clap::Parser)]
#[clap(bin_name = "cargo xtask")]
pub struct Args {
    #[clap(flatten)]
    verbosity: Verbosity,

    #[clap(subcommand)]
    command: Option<Command>,
}

impl Args {
    pub fn run(&self, config: &Config) -> eyre::Result<()> {
        match &self.command {
            Some(command) => command.run(config)?,
            None => Self::command().print_help()?,
        }
        Ok(())
    }

    pub fn verbosity(&self) -> Option<Level> {
        self.verbosity.get()
    }
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(flatten)]
    Command(cli_xtask::Command),
}

impl Command {
    fn run(&self, config: &Config) -> eyre::Result<()> {
        match self {
            Self::Command(args) => args.run(config),
        }
    }
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger(args.verbosity())?;

    tracing::info!("Running on {}", std::env::current_dir()?.display());
    let config = ConfigBuilder::new().build();
    args.run(&config)?;

    Ok(())
}
