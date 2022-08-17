use std::process::Command;

use cargo_metadata::Metadata;

use crate::fs::ToRelative;

/// Executes the given command at the given workspace root direcotory.
pub fn execute_on(
    metadata: &Metadata,
    command: impl AsRef<str>,
    args: impl IntoIterator<Item = impl Into<String>>,
) -> eyre::Result<()> {
    let command = command.as_ref();
    let args = args.into_iter().map(Into::into).collect::<Vec<_>>();

    let workspace_root = &metadata.workspace_root;
    tracing::info!(
        "[{}]$ {} {}",
        workspace_root.to_relative(),
        command,
        args.join(" ")
    );
    let status = Command::new(command)
        .args(args)
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
    Ok(())
}
