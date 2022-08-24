use std::process::Command;

use crate::{
    args::{EnvArgs, FeatureArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `test` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-test.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Test {
    /// Environment variables to set for `cargo test`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
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
            env_args,
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // cargo test --package <pkg> <features> <extra_options>
            // DO NOT USE `--all-targets` here, doctests are not built with `--all-targets`
            Command::new("cargo")
                .args(["test", "--package", &package.name])
                .args(features.map(|f| f.to_args()).unwrap_or_default())
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
