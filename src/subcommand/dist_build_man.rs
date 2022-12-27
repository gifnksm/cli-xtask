use std::{any::Any, iter};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap_mangen::Man;
use time::OffsetDateTime;

use crate::{config::Config, Result, Run};

/// Arguments definition of the `dist-build-man` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build-man.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuildMan {}

impl Run for DistBuildMan {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
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

impl DistBuildMan {
    /// Execute `dist-build-man` subcommand workflow
    #[tracing::instrument(name = "dist-build-man", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        tracing::info!("Building man pages...");

        let Self {} = self;
        let config = config.dist()?;

        let man_dir = config.dist_working_directory(None).join("man");
        let section = "1";
        crate::fs::remove_dir(&man_dir)?;

        for package in config.packages() {
            for target in package.targets() {
                if let Some(cmd) = target.command() {
                    let it = dist_build_man_pages(&man_dir, package.name(), cmd.clone(), section)?;
                    for res in it {
                        let (path, man) = res?;
                        let mut file = crate::fs::create_file(path)?;
                        man.render(&mut file)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn dist_build_man_pages(
    man_dir: &Utf8Path,
    package_name: &str,
    cmd: clap::Command,
    section: impl Into<String>,
) -> Result<impl Iterator<Item = Result<(Utf8PathBuf, Man)>>> {
    let capitalized_name = {
        let mut cs = package_name.chars();
        match cs.next() {
            Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
            None => String::new(),
        }
    };
    let section = section.into();

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
        cmd.get_name().replace(' ', "-"),
        cmd.get_version()
            .or_else(|| cmd.get_long_version())
            .unwrap_or_default()
    );

    let man_dir = man_dir.to_owned();
    let it = iterate_commands(cmd).map(move |cmd| {
        let command_name = cmd.get_name().replace(' ', "-");
        let filename = format!("{command_name}.{}", section);
        let path = man_dir.join(filename);
        let man = Man::new(cmd)
            .title(command_name.to_uppercase())
            .section(&section)
            .date(&date)
            .source(&source)
            .manual(&manual_name);
        Ok((path, man))
    });

    Ok(it)
}

fn iterate_commands(cmd: clap::Command) -> Box<dyn Iterator<Item = clap::Command>> {
    #[allow(clippy::needless_collect)]
    let subcommands = cmd.get_subcommands().cloned().collect::<Vec<_>>();
    let command_name = cmd.get_name().to_string();
    let command_version = cmd.get_version().map(str::to_string);
    let command_long_version = cmd.get_long_version().map(str::to_string);
    let command_author = cmd.get_author().map(str::to_string);

    let it = iter::once(cmd).chain(
        subcommands
            .into_iter()
            .map(move |mut subcommand| {
                let name = format!("{command_name} {}", subcommand.get_name());
                subcommand = subcommand.name(name);
                if subcommand.get_version().is_none() {
                    if let Some(version) = &command_version {
                        subcommand = subcommand.version(version);
                    }
                }
                if subcommand.get_long_version().is_none() {
                    if let Some(long_version) = &command_long_version {
                        subcommand = subcommand.long_version(long_version);
                    }
                }
                if subcommand.get_author().is_none() {
                    if let Some(author) = &command_author {
                        subcommand = subcommand.author(author);
                    }
                }
                subcommand
            })
            .flat_map(iterate_commands),
    );
    Box::new(it)
}
