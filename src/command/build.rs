use clap::Parser;

use crate::{config::Config, process, workspace};

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

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                for feature_args in workspace::feature_combination_args(package) {
                    // cargo build --package <pkg> <features> <extra_options>
                    process::execute_on(
                        metadata,
                        "cargo",
                        ["build", "--package", &package.name]
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
