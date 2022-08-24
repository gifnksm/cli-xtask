use std::process::Command;

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
}

impl Udeps {
    /// Runs the `udeps` subcommand.
    #[tracing::instrument(name = "udeps", parent = None, skip_all, err)]
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
                .args(
                    [
                        "run",
                        "nightly",
                        "cargo",
                        "udeps",
                        "--package",
                        &package.name,
                    ]
                    .into_iter()
                    .chain(features.map(|f| f.to_args()).unwrap_or_default())
                    .chain(extra_options.iter().map(String::as_str)),
                )
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
