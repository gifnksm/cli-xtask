use std::process::{Command, Stdio};

use cargo_metadata::Metadata;

use crate::fs::ToRelative;

/// Extension methods for [`std::process::Command`].
pub trait CommandExt {
    /// Executes the command as a child process on the workspace root directory, waiting for it to finish and checking the exit status.
    fn workspace_spawn(&mut self, workspace: &Metadata) -> eyre::Result<()>;

    /// Executes the command as a child process on the workspace root directory, waiting for it to finish and collecting all of its standard output.
    fn workspace_stdout(&mut self, workspace: &Metadata) -> eyre::Result<Vec<u8>>;
}

impl CommandExt for Command {
    fn workspace_spawn(&mut self, workspace: &Metadata) -> eyre::Result<()> {
        let workspace_root = &workspace.workspace_root;

        self.current_dir(&workspace_root);

        let program = self.get_program();
        let args = self.get_args();
        tracing::info!(
            "[{}]$ {}{}",
            workspace.workspace_root.to_relative(),
            program.to_string_lossy(),
            args.fold(String::new(), |mut s, a| {
                s.push(' ');
                s.push_str(a.to_string_lossy().as_ref());
                s
            })
        );

        let status = self.status()?;
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
        Ok(())
    }

    fn workspace_stdout(&mut self, workspace: &Metadata) -> eyre::Result<Vec<u8>> {
        let workspace_root = &workspace.workspace_root;

        self.current_dir(&workspace_root).stdout(Stdio::piped());

        let program = self.get_program();
        let args = self.get_args();
        tracing::info!(
            "[{}]$ {}{}",
            workspace.workspace_root.to_relative(),
            program.to_string_lossy(),
            args.fold(String::new(), |mut s, a| {
                s.push(' ');
                s.push_str(a.to_string_lossy().as_ref());
                s
            })
        );

        let output = self.spawn()?.wait_with_output()?;

        if !output.status.success() {
            tracing::error!(
                "Command for {} failed with status {}",
                workspace_root.to_relative(),
                output.status.code().unwrap()
            );
            return Err(eyre::eyre!(
                "command for {} failed with status {}",
                workspace_root.to_relative(),
                output.status.code().unwrap()
            ));
        }

        Ok(output.stdout)
    }
}
