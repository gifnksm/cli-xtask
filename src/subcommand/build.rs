use std::{any::Any, process::Command};

use crate::{
    args::{EnvArgs, FeatureArgs},
    config::Config,
    process::CommandExt,
    Result, Run, SubcommandRun,
};

/// Arguments definition of the `build` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-build.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Build {
    /// Environment variables to set for `cargo build`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
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

    fn to_subcommands(&self) -> Option<SubcommandRun> {
        None
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Build {
    /// Runs the `build` subcommand.
    #[tracing::instrument(name = "build", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // cargo build --package <pkg> <features> <extra_options>
            Command::new("cargo")
                .args(["build", "--package", &package.name])
                .args(features.map(|f| f.to_args()).unwrap_or_default())
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
