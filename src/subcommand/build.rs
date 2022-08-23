use std::process::Command;

use crate::{args::FeatureArgs, config::Config, process::CommandExt, Result, Run};

/// Arguments definition of the `build` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-build.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Build {
    /// Features to run the `cargo build` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo build`
    pub extra_options: Vec<String>,
}

impl Run for Build {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Build {
    /// Runs the `build` subcommand.
    #[tracing::instrument(name = "build", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // cargo build --package <pkg> <features> <extra_options>
            Command::new("cargo")
                .args(
                    ["build", "--package", &package.name]
                        .into_iter()
                        .chain(features.map(|f| f.to_args()).unwrap_or_default())
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
