use clap::Parser;

use crate::{config::Config, process, workspace};

/// `fmt` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Fmt {
    /// Arguments to pass to the `cargo fmt`
    extra_options: Vec<String>,
}

impl Fmt {
    /// Execute `fmt` subcommand workflow.
    #[tracing::instrument(name = "fmt", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                process::execute_on(
                    metadata,
                    "cargo",
                    ["fmt", "--package", &package.name]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )?;
            }
        }

        Ok(())
    }
}
