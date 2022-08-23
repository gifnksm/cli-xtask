use std::process::Command;

use crate::{args::FeatureArgs, config::Config, process::CommandExt, Result, Run};

/// Arguments definition of the `test` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-test.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Test {
    /// Features to run the `cargo test` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo test`
    pub extra_options: Vec<String>,
}

impl Run for Test {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Test {
    /// Runs the `test` subcommand.
    #[tracing::instrument(name = "test", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // cargo test --package <pkg> <features> <extra_options>
            // DO NOT USE `--all-targets` here, doctests are not built with `--all-targets`
            Command::new("cargo")
                .args(
                    ["test", "--package", &package.name]
                        .into_iter()
                        .chain(features.map(|f| f.to_args()).unwrap_or_default())
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
