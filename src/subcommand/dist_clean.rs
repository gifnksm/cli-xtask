use std::any::Any;

use crate::{config::Config, Result, Run, SubcommandRun};

/// Arguments definition of the `dist-clean` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-clean.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistClean {}

impl Run for DistClean {
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

impl DistClean {
    /// Runs the `dist-clean` subcommand.
    #[tracing::instrument(name = "dist-clean", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {} = self;
        let config = config.dist()?;

        let dist_dir = config.dist_target_directory();
        crate::fs::remove_dir(&dist_dir)?;

        let working_dir = config.dist_base_working_directory();
        crate::fs::remove_dir(&working_dir)?;

        Ok(())
    }
}
