use std::process::Command;

use crate::{config::Config, process::CommandExt, workspace, Result, Run};

/// `rdme` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Rdme {
    /// Arguments to pass to the `cargo rdme`
    pub extra_options: Vec<String>,
}

impl Run for Rdme {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Rdme {
    /// Execute `rdme` subcommand workflow.
    #[tracing::instrument(name = "rdme", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            Command::new("cargo")
                .args(
                    ["rdme"]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
