use std::fs;

use clap::Parser;

use crate::{archive, DistConfig};

/// `dist` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Dist {
    #[cfg(command_dist_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
    #[clap(flatten)]
    dist_build_args: super::DistBuild,
}

impl Dist {
    /// Execute `dist` subcommand workflow.
    #[tracing::instrument(name = "dist", parent = None, skip_all, err)]
    pub(crate) fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        let Self {
            #[cfg(command_dist_build)]
            dist_build_args,
        } = self;

        #[cfg(command_dist_build)]
        dist_build_args.run(config)?;

        let dist_dir = config.dist_target_directory();
        fs::create_dir_all(&dist_dir)?;

        let noarch_path = config.dist_base_working_directory().join("noarch");
        let noarch_path = noarch_path.is_dir().then(|| noarch_path);

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
        }

        Ok(())
    }
}
