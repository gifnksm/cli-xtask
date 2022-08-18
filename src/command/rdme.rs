use std::process::Command;

use clap::Parser;

use crate::{config::Config, process::CommandExt, workspace};

/// `rdme` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Rdme {
    /// Arguments to pass to the `cargo rdme`
    extra_options: Vec<String>,
}

impl Rdme {
    /// Execute `rdme` subcommand workflow.
    #[tracing::instrument(name = "rdme", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
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
