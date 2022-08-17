use cli_xtask::{process, workspace};

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Do not execute command on the root workspace.
    #[clap(long, conflicts_with = "root-only")]
    no_root: bool,
    #[clap(long)]
    root_only: bool,
    /// Command to execute
    command: String,
    /// Arguments to pass to the command
    command_options: Vec<String>,
}

impl Args {
    #[tracing::instrument(name = "exec", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self {
            no_root,
            root_only,
            command,
            command_options,
        } = self;

        let current_workspace_root = &workspace::current().workspace_root;

        for metadata in workspace::all() {
            let is_root = &metadata.workspace_root == current_workspace_root;
            if is_root && *no_root {
                continue;
            }
            if !is_root && *root_only {
                continue;
            }
            process::execute_on(metadata, command, command_options)?;
        }

        Ok(())
    }
}
