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

        #[cfg(feature = "command-dist-build-bin")]
        let target_triple = dist_build_args
            .dist_build_bin_args
            .target_triple
            .as_deref()
            .unwrap_or(env!("DEFAULT_TARGET"));
        #[cfg(not(feature = "command-dist-build-bin"))]
        let target_triple = env!("DEFAULT_TARGET");

        let archive_name = format!("{}-{}.tar.gz", config.name(), target_triple);
        let archive_path = dist_dir.join(&archive_name);

        archive::create(&archive_path, config.dist_working_directory())?;

        tracing::info!("Archive created successfully: {archive_path}");

        Ok(())
    }
}
