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

        let current_workspace_root = &cli_xtask::current_workspace().workspace_root;

        for (workspace_root, metadata) in crate::all_workspaces()? {
            let is_root = &workspace_root == current_workspace_root;
            if is_root && *no_root {
                continue;
            }
            if !is_root && *root_only {
                continue;
            }
            crate::execute_on(&metadata, command, command_options)?;
        }

        Ok(())
    }
}
