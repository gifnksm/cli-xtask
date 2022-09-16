use std::{any::Any, process::Command};

use crate::{
    args::{EnvArgs, PackageArgs},
    config::Config,
    process::CommandExt,
    Result, Run, SubcommandRun,
};

/// Arguments definition of the `fmt` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-fmt.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Fmt {
    /// Environment variables to set for `cargo fmt`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Packages to run the `cargo fmt` for
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Options to pass to the `cargo fmt`
    pub extra_options: Vec<String>,
}

impl Run for Fmt {
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

impl Fmt {
    /// Runs the `fmt` subcommand.
    #[tracing::instrument(name = "fmt", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            package_args,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            // cargo fmt --package <pkg> <extra_options>
            Command::new("cargo")
                .args(["fmt", "--package", &package.name])
                .args(extra_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
