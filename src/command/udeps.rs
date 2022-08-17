use clap::Parser;

use crate::{process, workspace, Config};

/// `udeps` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Udeps {
    /// Arguments to pass to the `cargo udeps`
    extra_options: Vec<String>,
}

impl Udeps {
    /// Execute `udeps` subcommand workflow.
    #[tracing::instrument(name = "udeps", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    // `cargo +nightly udeps` fails on windows, so use rustup instead
                    process::execute_on(
                        metadata,
                        "rustup",
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
                    )?;
                }
            }
        }

        Ok(())
    }
}
