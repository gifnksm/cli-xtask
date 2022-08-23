use std::process::Command;

use crate::{args::WorkspaceArgs, config::Config, process::CommandExt, Result, Run};

/// Arguments definition of the `rdme` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-rdme.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Rdme {
    /// Workspaces where the `cargo rdme` runs on
    #[clap(flatten)]
    pub workspace_args: WorkspaceArgs,
    /// Options to pass to the `cargo rdme`
    pub extra_options: Vec<String>,
}

impl Run for Rdme {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Rdme {
    /// Runs the `rdme` subcommand.
    #[tracing::instrument(name = "rdme", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            workspace_args,
            extra_options,
        } = self;

        for workspace in workspace_args.workspaces() {
            // cargo rdme <extra_options>
            Command::new("cargo")
                .args(
                    ["rdme"]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
