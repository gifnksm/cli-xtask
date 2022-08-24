use std::process::Command;

use crate::{args::PackageArgs, config::Config, process::CommandExt, Result, Run};

/// Arguments definition of the `doc` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-doc.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Doc {
    /// Packages to run the `cargo doc` with
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Options to pass to the `cargo build`
    pub extra_options: Vec<String>,
}

impl Run for Doc {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Doc {
    /// Runs the `doc` subcommand.
    #[tracing::instrument(name = "doc", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            package_args,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            // cargo doc --package <pkg> <features> <extra_options>
            Command::new("cargo")
                .args(
                    ["doc", "--package", &package.name, "--all-features"]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
