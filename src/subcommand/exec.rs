use std::process::Command;

use crate::{
    args::{EnvArgs, WorkspaceArgs},
    config::Config,
    process::CommandExt,
    Result, Run,
};

/// Arguments definition of the `exec` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-exec.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Exec {
    /// Environment variables to set for the command.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Workspaces where the the command runs on.
    #[clap(flatten)]
    pub workspace_args: WorkspaceArgs,
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
            env_args,
            workspace_args,
            command,
            command_options,
        } = self;

        for workspace in workspace_args.workspaces() {
            Command::new(command)
                .args(command_options)
                .envs(env_args.env.clone())
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
