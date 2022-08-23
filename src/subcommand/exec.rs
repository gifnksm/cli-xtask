use std::process::Command;

use crate::{config::Config, process::CommandExt, workspace, Result, Run};

/// Arguments definition of the `exec` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-exec.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Exec {
    /// Do not execute command on the current workspace.
    #[clap(long)]
    pub exclude_current: bool,
    /// Command to execute
    pub command: String,
    /// Arguments to pass to the command
    pub command_options: Vec<String>,
}

impl Run for Exec {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Exec {
    /// Runs the `exec` subcommand.
    #[tracing::instrument(name = "exec", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            exclude_current,
            command,
            command_options,
        } = self;

        let workspaces = workspace::all().iter().filter(|ws| {
            !*exclude_current || ws.workspace_root != workspace::current().workspace_root
        });

        for workspace in workspaces {
            Command::new(command)
                .args(command_options)
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
