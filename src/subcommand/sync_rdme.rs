use std::process::Command;

use crate::{
    args::{EnvArgs, PackageArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `sync-rdme` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-sync-rdme.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct SyncRdme {
    /// Environment variables to set for `cargo sync-rdme`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Packages to run the `cargo sync-rdme` with.
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Options to pass to the `cargo sync-rdme`
    pub extra_options: Vec<String>,
}

impl Run for SyncRdme {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl SyncRdme {
    /// Runs the `sync-rdme` subcommand.
    #[tracing::instrument(name = "sync-rdme", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            package_args,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            // rustup run nightly cargo sync-rdme <extra_options>
            // `cargo +nightly sync-rdme` fails on windows, so use rustup instead
            // cargo sync-rdme <extra_options>
            Command::new("rustup")
                .args([
                    "run",
                    "nightly",
                    "cargo",
                    "sync-rdme",
                    "--package",
                    &package.name,
                    "--all-features",
                ])
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
