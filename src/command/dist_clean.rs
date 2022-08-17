use clap::Parser;

use crate::DistConfig;

/// `dist-clean` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistClean {}

impl DistClean {
    /// Execute `dist-clean` subcommand workflow.
    #[tracing::instrument(name = "dist-clean", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        let Self {} = self;

        let dist_dir = config.dist_target_directory();
        crate::fs::remove_dir(&dist_dir)?;

        let working_dir = config.dist_base_working_directory();
        crate::fs::remove_dir(&working_dir)?;

        Ok(())
    }
}
