use std::process::Command;

use clap::Parser;

use crate::{config::Config, process::CommandExt, workspace};

/// `build` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Build {
    /// Arguments to pass to the `cargo build`
    extra_options: Vec<String>,
}

impl Build {
    /// Execute `build` subcommand workflow.
    #[tracing::instrument(name = "build", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            for package in workspace.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    // cargo build --package <pkg> <features> <extra_options>
                    Command::new("cargo")
                        .args(
                            ["build", "--package", &package.name]
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
