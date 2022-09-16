use std::any::Any;

use crate::{config::Config, Result, Run, SubcommandRun};

/// Arguments definition of the `dist` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Dist {
    /// Arguments for the `dist-build` subcommand.
    #[cfg(subcommand_dist_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-*")))]
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

impl Dist {
    /// Runs the `dist` subcommand.
    #[tracing::instrument(name = "dist", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {
            #[cfg(subcommand_dist_build)]
            dist_build_args,
            dist_archive_args,
        } = self;
        let dist_config = config.dist()?;

        let working_dir = dist_config.dist_base_working_directory();
        crate::fs::create_or_cleanup_dir(&working_dir)?;

        #[cfg(subcommand_dist_build)]
        dist_build_args.run(config)?;

        dist_archive_args.run(config)?;

        Ok(())
    }
}
