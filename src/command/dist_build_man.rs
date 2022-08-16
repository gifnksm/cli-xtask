use std::{fs, iter};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use clap_mangen::Man;
use time::OffsetDateTime;

use crate::DistConfig;

/// `dist-build-man` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuildMan {}

impl DistBuildMan {
    /// Execute `dist-build-man` subcommand workflow
    #[tracing::instrument(name = "dist-build-man", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        tracing::info!("Building man pages...");

        let Self {} = self;

        let man_dir = config.dist_working_directory(None).join("share/man");
        let section = "1";

        crate::fs::create_or_cleanup_dir(&man_dir)?;

        for package in config.packages() {
            for target in package.targets().into_iter().flatten() {
                if let Some(cmd) = target.command() {
                    let it = dist_build_man_pages(&man_dir, package.name(), cmd.clone(), section)?;
                    for res in it {
                        let (path, man) = res?;
                        let mut file = crate::fs::create_file(&path)?;
                        man.render(&mut file)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn dist_build_man_pages<'a>(
    man_dir: &Utf8Path,
    package_name: &str,
    mut cmd: clap::Command<'a>,
    section: impl Into<String>,
) -> eyre::Result<impl Iterator<Item = eyre::Result<(Utf8PathBuf, Man<'a>)>> + 'a> {
    cmd._build_all(); // https://github.com/clap-rs/clap/discussions/3603

    let capitalized_name = {
        let mut cs = package_name.chars();
        match cs.next() {
            Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
            None => String::new(),
        }
    };
    let section = section.into();
    let out_dir = man_dir.join(format!("man{section}"));
    fs::create_dir_all(&out_dir)?;

    let now = OffsetDateTime::now_utc();
    let manual_name = format!("{capitalized_name} Command Manual");
    let date = format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    );
    let source = format!(
        "{} {}",
        cmd.get_name(),
        cmd.get_version()
            .or_else(|| cmd.get_long_version())
            .unwrap_or_default()
    );

    let it = iterate_commands(cmd).map(move |cmd| {
        let command_name = cmd.get_name().to_string();
        let filename = format!("{command_name}.{}", section);
        let path = out_dir.join(&filename);
        let man = Man::new(cmd.clone())
            .title(command_name.to_uppercase())
            .section(&section)
            .date(&date)
            .source(&source)
            .manual(&manual_name);
        Ok((path, man))
    });

    Ok(it)
}

fn iterate_commands<'a>(
    cmd: clap::Command<'a>,
) -> Box<dyn Iterator<Item = clap::Command<'a>> + 'a> {
    #[allow(clippy::needless_collect)]
    let subcommands = cmd.get_subcommands().cloned().collect::<Vec<_>>();
    let command_name = cmd.get_name().to_string();
    let command_version = cmd.get_version();
    let command_long_version = cmd.get_long_version();
    let command_author = cmd.get_author();

    let it = iter::once(cmd.clone()).chain(
        subcommands
            .into_iter()
            .map(move |mut subcommand| {
                let name = format!("{command_name}-{}", subcommand.get_name());
                subcommand = subcommand.name(name);
                if subcommand.get_version().is_none() {
                    if let Some(version) = command_version {
                        subcommand = subcommand.version(version);
                    }
                }
                if subcommand.get_long_version().is_none() {
                    if let Some(long_version) = command_long_version {
                        subcommand = subcommand.long_version(long_version);
                    }
                }
                if subcommand.get_author().is_none() {
                    if let Some(author) = command_author {
                        subcommand = subcommand.author(author);
                    }
                }
                subcommand
            })
            .flat_map(iterate_commands),
    );
    Box::new(it)
}
