use std::any::Any;

use crate::{archive, config::Config, Result, Run};

/// Arguments definition of the `dist-archive` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-archive.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistArchive {}

impl Run for DistArchive {
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

impl DistArchive {
    /// Runs the `dist-archive` subcommand.
    #[tracing::instrument(name = "dist-archive", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {} = self;
        let config = config.dist()?;

        if !config.dist_base_working_directory().is_dir() {
            tracing::warn!("no build artifacts found");
            return Ok(());
        }

        let dist_dir = config.dist_target_directory();

        let noarch_path = config.dist_base_working_directory().join("noarch");
        let noarch_path = noarch_path.is_dir().then_some(noarch_path);

        let mut created = false;
        for dir in config.dist_base_working_directory().read_dir_utf8()? {
            let dir = dir?;
            if !dir.file_type()?.is_dir() {
                continue;
            }
            let dir = dir.path();
            if dir.file_name() == Some("noarch") {
                continue;
            }
            let target_triple = dir.file_name().unwrap();
            let archive_name = format!("{}-{}.tar.gz", config.name(), target_triple);
            let archive_path = dist_dir.join(&archive_name);

            let mut targets = vec![];
            for dir in [dir].into_iter().chain(noarch_path.as_deref()) {
                for entry in dir.read_dir_utf8()? {
                    targets.push(entry?.path().to_owned());
                }
            }
            archive::create(&archive_path, &targets)?;

            tracing::info!("Archive created successfully: {archive_path}");
            created = true;
        }

        if !created && noarch_path.is_some() {
            let archive_name = format!("{}-noarch.tar.gz", config.name());
            let archive_path = dist_dir.join(&archive_name);

            archive::create(&archive_path, [noarch_path.unwrap()].into_iter())?;

            tracing::info!("Archive created successfully: {archive_path}");
        }

        Ok(())
    }
}
