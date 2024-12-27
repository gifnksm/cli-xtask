use std::fmt;

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap_complete::Generator;

use crate::{config::Config, fs::ToRelative, Result, Run};

/// Arguments definition of the `dist-build-completion` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build-completion.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuildCompletion {}

impl Run for DistBuildCompletion {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

#[derive(Debug, Clone, Copy)]
enum Shell {
    Bash,
    Elvish,
    Fish,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
    Zsh,
    Nushell,
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => fmt::Display::fmt(&clap_complete::Shell::Bash, f),
            Shell::Elvish => fmt::Display::fmt(&clap_complete::Shell::Elvish, f),
            Shell::Fish => fmt::Display::fmt(&clap_complete::Shell::Fish, f),
            Shell::PowerShell => fmt::Display::fmt(&clap_complete::Shell::PowerShell, f),
            Shell::Zsh => fmt::Display::fmt(&clap_complete::Shell::Zsh, f),
            Shell::Nushell => fmt::Display::fmt("nushell", f),
        }
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => Generator::file_name(&clap_complete::Shell::Bash, name),
            Shell::Elvish => Generator::file_name(&clap_complete::Shell::Elvish, name),
            Shell::Fish => Generator::file_name(&clap_complete::Shell::Fish, name),
            Shell::PowerShell => Generator::file_name(&clap_complete::Shell::PowerShell, name),
            Shell::Zsh => Generator::file_name(&clap_complete::Shell::Zsh, name),
            Shell::Nushell => Generator::file_name(&clap_complete_nushell::Nushell, name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => Generator::generate(&clap_complete::Shell::Bash, cmd, buf),
            Shell::Elvish => Generator::generate(&clap_complete::Shell::Elvish, cmd, buf),
            Shell::Fish => Generator::generate(&clap_complete::Shell::Fish, cmd, buf),
            Shell::PowerShell => Generator::generate(&clap_complete::Shell::PowerShell, cmd, buf),
            Shell::Zsh => Generator::generate(&clap_complete::Shell::Zsh, cmd, buf),
            Shell::Nushell => Generator::generate(&clap_complete_nushell::Nushell, cmd, buf),
        }
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
            Shell::Nushell,
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
    cmd: &clap::Command,
    bin_name: &str,
    out_dir: &Utf8Path,
) -> Result<Utf8PathBuf> {
    crate::fs::create_dir(out_dir)?;
    let path = clap_complete::generate_to(shell, &mut cmd.clone(), bin_name, out_dir)?;
    let path = Utf8PathBuf::try_from(path)?;
    tracing::info!("Generated {shell} completion file: {}", path.to_relative());
    Ok(path)
}
