use std::{any::Any, process::Command};

use crate::{
    args::{EnvArgs, PackageArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `doc` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-doc.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Doc {
    /// Environment variables to set for `cargo doc`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Packages to run the `cargo doc` with.
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Options to pass to the `cargo doc`.
    pub extra_options: Vec<String>,
}

impl Run for Doc {
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

impl Doc {
    /// Runs the `doc` subcommand.
    #[tracing::instrument(name = "doc", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            package_args,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            // cargo doc --package <pkg> <features> <extra_options>
            Command::new("cargo")
                .args(["doc", "--package", &package.name, "--all-features"])
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
