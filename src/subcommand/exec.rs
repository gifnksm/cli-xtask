use std::process::Command;

use crate::{
    Result, Run,
    args::{EnvArgs, WorkspaceArgs},
    config::Config,
    process::CommandExt,
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
    #[tracing::instrument(name = "exec", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            workspace_args,
            command,
            command_options,
        } = self;

        let is_cargo_llvm_cov =
            command == "cargo" && command_options.first().is_some_and(|arg| arg == "llvm-cov");

        for workspace in workspace_args.workspaces() {
            let mut cmd = Command::new(command);
            cmd.args(command_options).envs(env_args.env.clone());

            if is_cargo_llvm_cov {
                // Avoid cargo-llvm-cov recursion when running under llvm-cov.
                cmd.env_remove("RUSTC_WRAPPER")
                    .env_remove("RUSTC_WORKSPACE_WRAPPER");
            }

            cmd.workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
