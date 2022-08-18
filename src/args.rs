use clap::Parser;
use tracing::Level;

use crate::config::Config;

/// Rust project automation command.
#[derive(Debug, Parser)]
#[clap(bin_name = "cargo xtask")]
pub struct Args {
    #[clap(flatten)]
    verbosity: Verbosity,

    #[cfg(command)]
    #[clap(subcommand)]
    command: Option<crate::Command>,
}

impl Args {
    /// Runs xtask workflow.
    pub fn run(&self, config: &Config) -> eyre::Result<()> {
        let _config = config; // suppress unused-var warnings

        #[cfg(command)]
        match &self.command {
            Some(command) => command.run(config)?,
            None => <Self as clap::CommandFactory>::command().print_help()?,
        }

        Ok(())
    }

    /// Returns the log verbosity level.
    pub fn verbosity(&self) -> Option<Level> {
        self.verbosity.get()
    }
}

/// Commmand line arguments to control log verbosity level.
#[derive(Debug, Parser)]
pub struct Verbosity {
    /// More output per occurrence
    #[clap(long, short = 'v', parse(from_occurrences), global = true)]
    verbose: i8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        parse(from_occurrences),
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: i8,
}

impl Verbosity {
    /// Returns the log verbosity level.
    pub fn get(&self) -> Option<Level> {
        let level = self.verbose - self.quiet;
        match level {
            i8::MIN..=-3 => None,
            -2 => Some(Level::ERROR),
            -1 => Some(Level::WARN),
            0 => Some(Level::INFO),
            1 => Some(Level::DEBUG),
            2..=i8::MAX => Some(Level::TRACE),
        }
    }
}
