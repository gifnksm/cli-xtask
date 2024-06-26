use std::process::Command;

use crate::{
    args::{EnvArgs, FeatureArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `clippy` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-clippy.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Clippy {
    /// Environment variables to set for `cargo clippy`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Features to run the `cargo clippy` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo clippy`
    pub extra_options: Vec<String>,
}

impl Run for Clippy {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Clippy {
    /// Runs the `clippy` subcommand.
    #[tracing::instrument(name = "clippy", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            Command::new("cargo")
                .args(["clippy", "--package", &package.name])
                .args(features.map(|f| f.to_args()).unwrap_or_default())
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
