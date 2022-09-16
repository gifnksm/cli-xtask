use std::any::Any;

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap_complete::Shell;

use crate::{config::Config, fs::ToRelative, Result, Run, SubcommandRun};

/// Arguments definition of the `dist-build-completion` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build-completion.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuildCompletion {}

impl Run for DistBuildCompletion {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }

    fn to_subcommands(&self) -> Option<SubcommandRun> {
        None
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl DistBuildCompletion {
    /// Runs the `dist-build-completion` subcommand.
    #[tracing::instrument(name = "dist-build-completion", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building shell completion files...");

        let Self {} = self;
        let config = config.dist()?;

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
            for target in package.targets() {
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
) -> Result<Utf8PathBuf> {
    crate::fs::create_dir(&out_dir)?;
    let path = clap_complete::generate_to(shell, &mut cmd.clone(), bin_name, &out_dir)?;
    let path = Utf8PathBuf::try_from(path)?;
    tracing::info!("Generated {shell} completion file: {}", path.to_relative());
    Ok(path)
}
