use std::process::Command;

use crate::{config::Config, process::CommandExt, workspace, Result, Run};

/// `udeps` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Udeps {
    /// Arguments to pass to the `cargo udeps`
    pub extra_options: Vec<String>,
}

impl Run for Udeps {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Udeps {
    /// Execute `udeps` subcommand workflow.
    #[tracing::instrument(name = "udeps", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            for package in workspace.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    // `cargo +nightly udeps` fails on windows, so use rustup instead
                    Command::new("rustup")
                        .args(
                            [
                                "run",
                                "nightly",
                                "cargo",
                                "udeps",
                                "--package",
                                &package.name,
                            ]
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
