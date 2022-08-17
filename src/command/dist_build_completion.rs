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

        let out_dir = config.dist_working_directory(None).join("completion");
        crate::fs::remove_dir(&out_dir)?;

        let shells = [
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
        ];

        for package in config.packages() {
            for target in package.targets().into_iter().flatten() {
                let target_name = target.name();
                if let Some(cmd) = target.command() {
                    for shell in shells {
                        generate(shell, cmd, target_name, &out_dir)?;
                    }
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
    out_dir: &Utf8Path,
) -> eyre::Result<Utf8PathBuf> {
    crate::fs::create_dir(&out_dir)?;
    let path = clap_complete::generate_to(shell, &mut cmd.clone(), bin_name, &out_dir)?;
    let path = Utf8PathBuf::try_from(path)?;
    tracing::info!("Generated {shell} completion file: {}", path.to_relative());
    Ok(path)
}
