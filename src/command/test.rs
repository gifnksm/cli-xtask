use std::process::Command;

use clap::Parser;

use crate::{config::Config, process::CommandExt, workspace};

/// `test` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Test {
    /// Arguments to pass to the `cargo test`
    extra_options: Vec<String>,
}

impl Test {
    /// Execute `test` subcommand workflow.
    #[tracing::instrument(name = "test", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for workspace in workspace::all() {
            for package in workspace.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    // cargo test --package <pkg> <features> <extra_options>
                    // DO NOT USE `--all-targets` here, doctests are not built with `--all-targets`
                    Command::new("cargo")
                        .args(
                            ["test", "--package", &package.name]
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
