use std::process::Command;

use crate::{config::Config, process::CommandExt, workspace, Result, Run};

/// `clippy` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Clippy {
    /// Arguments to pass to the `cargo clippy`
    pub extra_options: Vec<String>,
}

impl Run for Clippy {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Clippy {
    /// Execute `clippy` subcommand workflow.
    #[tracing::instrument(name = "clippy", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            for package in workspace.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    Command::new("cargo")
                        .args(
                            ["clippy", "--all-targets", "--package", &package.name]
                                .into_iter()
                                .chain(feature_args.iter().copied())
                                .chain(extra_options.iter().map(String::as_str)),
                        )
                        .workspace_spawn(workspace)?;
                }
            }
        }

        Ok(())
    }
}
