use std::process::Command;

use clap::Parser;

use crate::{config::Config, process::CommandExt, workspace};

/// `exec` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Exec {
    /// Do not execute command on the current workspace.
    #[clap(long)]
    exclude_current: bool,
    /// Command to execute
    command: String,
    /// Arguments to pass to the command
    command_options: Vec<String>,
}

impl Exec {
    /// Execute `exec` subcommand workflow.
    #[tracing::instrument(name = "exec", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> eyre::Result<()> {
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
