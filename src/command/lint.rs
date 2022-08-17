use clap::Parser;

use crate::Config;

/// `lint` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Lint {}

impl Lint {
    /// Execute `lint` subcommand workflow.
    #[tracing::instrument(name = "lint", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> eyre::Result<()> {
        let Self {} = self;

        let _ = config; // supress unused-variables warning

        // cargo fmt --check
        #[cfg(feature = "command-fmt")]
        crate::command::Fmt::parse_from(["fmt", "--", "--check"]).run(config)?;
        // cargo clippy -- -D warnings
        #[cfg(feature = "command-clippy")]
        crate::command::Clippy::parse_from(["clippy", "--", "--", "-D", "warnings"]).run(config)?;
        // cargo rdme --check
        #[cfg(feature = "command-rdme")]
        crate::command::Rdme::parse_from(["rdme", "--", "--check"]).run(config)?;
        // cargo udeps
        #[cfg(feature = "command-udeps")]
        crate::command::Udeps::parse_from(["udeps"]).run(config)?;

        Ok(())
    }
}
