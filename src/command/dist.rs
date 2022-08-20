use crate::{config::Config, Result, Run};

/// `dist` subcommand arguments.
#[derive(Debug, clap::Args)]
pub struct Dist {
    /// Arguments for the `dist-build` subcommand.
    #[cfg(command_dist_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
    #[clap(flatten)]
    pub dist_build_args: super::DistBuild,
    /// Arguments for the `dist-archive` subcommand.
    #[clap(flatten)]
    pub dist_archive_args: super::DistArchive,
}

impl Run for Dist {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Dist {
    /// Execute `dist` subcommand workflow.
    #[tracing::instrument(name = "dist", parent = None, skip_all, err)]
    pub(crate) fn run(&self, config: &Config) -> Result<()> {
        let Self {
            #[cfg(command_dist_build)]
            dist_build_args,
            dist_archive_args,
        } = self;
        let dist_config = config.dist()?;

        let working_dir = dist_config.dist_base_working_directory();
        crate::fs::create_or_cleanup_dir(&working_dir)?;

        #[cfg(command_dist_build)]
        dist_build_args.run(config)?;

        dist_archive_args.run(config)?;

        Ok(())
    }
}
