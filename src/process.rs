//! Utility functions for working with processes.

use std::process::{Command, Stdio};

use cargo_metadata::Metadata;
use eyre::eyre;

use crate::{fs::ToRelative, Result};

/// Extension methods for [`std::process::Command`].
pub trait CommandExt {
    /// Executes the command as a child process on the workspace root directory,
    /// waiting for it to finish and checking the exit status.
    fn workspace_spawn(&mut self, workspace: &Metadata) -> Result<()>;

    /// Executes the command as a child process on the workspace root directory,
    /// waiting for it to finish and collecting all of its standard output as
    /// a bytes vector.
    fn workspace_stdout_raw(&mut self, workspace: &Metadata) -> Result<Vec<u8>>;

    /// Executes the command as a child process on the workspace root directory,
    /// waiting for it to finish and collecting all of its standard output as
    /// a string.
    fn workspace_stdout(&mut self, workspace: &Metadata) -> Result<String>;
}

impl CommandExt for Command {
    fn workspace_spawn(&mut self, workspace: &Metadata) -> Result<()> {
        let workspace_root = &workspace.workspace_root;

        self.current_dir(workspace_root);

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
                status,
            );
            return Err(eyre!(
                "command for {} failed with status {}",
                workspace_root.to_relative(),
                status,
            ));
        }
        Ok(())
    }

    fn workspace_stdout_raw(&mut self, workspace: &Metadata) -> Result<Vec<u8>> {
        let workspace_root = &workspace.workspace_root;

        self.current_dir(workspace_root).stdout(Stdio::piped());

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
            return Err(eyre!(
                "command for {} failed with status {}",
                workspace_root.to_relative(),
                output.status.code().unwrap()
            ));
        }

        Ok(output.stdout)
    }

    fn workspace_stdout(&mut self, workspace: &Metadata) -> Result<String> {
        let output = self.workspace_stdout_raw(workspace)?;
        Ok(String::from_utf8(output)?)
    }
}
