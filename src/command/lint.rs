use crate::{config::Config, Result, Run};

/// `lint` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Lint {}

impl Run for Lint {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Lint {
    /// Execute `lint` subcommand workflow.
    #[tracing::instrument(name = "lint", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {} = self;

        let _ = config; // supress unused-variables warning

        // cargo fmt --check
        #[cfg(feature = "command-fmt")]
        crate::command::Fmt {
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;
        // cargo clippy -- -D warnings
        #[cfg(feature = "command-clippy")]
        crate::command::Clippy {
            extra_options: ["--", "-D", "warnings"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
        .run(config)?;
        // cargo rdme --check
        #[cfg(feature = "command-rdme")]
        crate::command::Rdme {
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;
        // cargo udeps
        #[cfg(feature = "command-udeps")]
        crate::command::Udeps {
            extra_options: vec![],
        }
        .run(config)?;

        Ok(())
    }
}
