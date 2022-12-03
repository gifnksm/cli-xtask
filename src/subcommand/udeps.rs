use std::{any::Any, process::Command};

use crate::{
    args::{EnvArgs, FeatureArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `udeps` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-udeps.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Udeps {
    /// Environment variables to set for `cargo udeps`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Features to run the `cargo udeps` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo udeps`
    pub extra_options: Vec<String>,
}

impl Run for Udeps {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
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

impl Udeps {
    /// Runs the `udeps` subcommand.
    #[tracing::instrument(name = "udeps", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // rustup run nightly cargo udeps --package <pkg> <features> <extra_options>
            // `cargo +nightly udeps` fails on windows, so use rustup instead
            Command::new("rustup")
                .args([
                    "run",
                    "nightly",
                    "cargo",
                    "udeps",
                    "--package",
                    &package.name,
                    // workaround: on windows, `cargo udeps` fails for some packages with following error:
                    // error[E0514]: found crate `<crate>` compiled by an incompatible version of rustc
                    "--target-dir",
                    workspace.target_directory.join("nightly").as_str(),
                ])
                .args(features.map(|f| f.to_args()).unwrap_or_default())
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
