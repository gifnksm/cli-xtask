use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use clap_complete::Shell;

use crate::{fs::ToRelative, DistConfig};

/// `dist-build-completion` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuildCompletion {}

impl DistBuildCompletion {
    /// Execute `dist-build-completion` subcommand workflow.
    #[tracing::instrument(name = "dist-build-completion", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        tracing::info!("Building shell completion files...");

        let Self {} = self;

        let bash_dir = config
            .dist_working_directory(None)
            .join("share/bash-completion");
        let fish_dir = config
            .dist_working_directory(None)
            .join("share/fish/completions");
        let zsh_dir = config
            .dist_working_directory(None)
            .join("share/zsh/site-functions");

        crate::fs::create_or_cleanup_dir(&bash_dir)?;
        crate::fs::create_or_cleanup_dir(&fish_dir)?;
        crate::fs::create_or_cleanup_dir(&zsh_dir)?;

        for package in config.packages() {
            for target in package.targets().into_iter().flatten() {
                let target_name = target.name();
                if let Some(cmd) = target.command() {
                    generate(Shell::Bash, cmd, target_name, &bash_dir)?;
                    generate(Shell::Fish, cmd, target_name, &fish_dir)?;
                    generate(Shell::Zsh, cmd, target_name, &zsh_dir)?;
                }
            }
        }

        Ok(())
    }
}

fn generate(
    shell: Shell,
    cmd: &clap::Command<'_>,
    bin_name: &str,
    dir: &Utf8Path,
) -> eyre::Result<Utf8PathBuf> {
    let path = clap_complete::generate_to(shell, &mut cmd.clone(), bin_name, &dir)?;
    let path = Utf8PathBuf::try_from(path)?;
    tracing::info!("Generated {shell} completion file: {}", path.to_relative());
    Ok(path)
}
