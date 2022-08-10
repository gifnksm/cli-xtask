use std::process::Command;

use cli_xtask::fs::ToRelative;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Don's execute command on the root workspace.
    #[clap(long)]
    no_root: bool,
    /// Command to execute
    command: String,
    /// Arguments to pass to the command
    command_options: Vec<String>,
}

impl Args {
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self {
            no_root,
            command,
            command_options,
        } = self;

        let metadata = crate::metadata(!no_root)?;

        for metadata in metadata {
            let workspace_root = &metadata.workspace_root;
            tracing::info!("Executing command on {}", workspace_root.to_relative());
            tracing::info!("  $ {} {}", command, command_options.join(" "));
            let status = Command::new(command)
                .args(command_options)
                .current_dir(workspace_root)
                .status()?;
            if !status.success() {
                tracing::error!(
                    "Command for {} failed with status {}",
                    workspace_root.to_relative(),
                    status.code().unwrap()
                );
                return Err(eyre::eyre!(
                    "command for {} failed with status {}",
                    workspace_root.to_relative(),
                    status.code().unwrap()
                ));
            }
        }
        Ok(())
    }
}
