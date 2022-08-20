use std::process::Command;

use crate::{config::Config, process::CommandExt, workspace, Result, Run};

/// `fmt` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Fmt {
    /// Arguments to pass to the `cargo fmt`
    pub extra_options: Vec<String>,
}

impl Run for Fmt {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Fmt {
    /// Execute `fmt` subcommand workflow.
    #[tracing::instrument(name = "fmt", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            for package in workspace.workspace_packages() {
                Command::new("cargo")
                    .args(
                        ["fmt", "--package", &package.name]
                            .into_iter()
                            .chain(extra_options.iter().map(String::as_str)),
                    )
                    .workspace_spawn(workspace)?;
            }
        }

        Ok(())
    }
}
