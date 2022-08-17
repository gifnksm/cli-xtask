use clap::Parser;

use crate::{process, workspace, Config};

/// `clippy` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Clippy {
    /// Arguments to pass to the `cargo clippy`
    extra_options: Vec<String>,
}

impl Clippy {
    /// Execute `clippy` subcommand workflow.
    #[tracing::instrument(name = "clippy", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    process::execute_on(
                        metadata,
                        "cargo",
                        ["clippy", "--all-targets", "--package", &package.name]
                            .into_iter()
                            .chain(feature_args.iter().copied())
                            .chain(extra_options.iter().map(String::as_str)),
                    )?;
                }
            }
        }

        Ok(())
    }
}
