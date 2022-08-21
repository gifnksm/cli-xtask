use std::process::Command;

use crate::{args::PackageArgs, config::Config, process::CommandExt, Result, Run};

/// `fmt` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Fmt {
    /// Packages to run the `cargo fmt` for
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Options to pass to the `cargo fmt`
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
        let Self {
            package_args,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            // cargo fmt --package <pkg> <extra_options>
            Command::new("cargo")
                .args(
                    ["fmt", "--package", &package.name]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
