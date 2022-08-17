use std::fs;

use clap::Parser;

use crate::{archive, DistConfig};

/// `dist-archive` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistArchive {}

impl DistArchive {
    /// Execute `dist-archive` subcommand workflow.
    #[tracing::instrument(name = "dist-archive", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        let Self {} = self;

        let dist_dir = config.dist_target_directory();
        fs::create_dir_all(&dist_dir)?;

        let noarch_path = config.dist_base_working_directory().join("noarch");
        let noarch_path = noarch_path.is_dir().then(|| noarch_path);

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

            archive::create(
                &archive_path,
                [dir].into_iter().chain(noarch_path.as_deref()),
            )?;

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
