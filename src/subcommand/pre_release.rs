use crate::{config::Config, Result, Run};

/// `pre-release` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct PreRelease {}

impl Run for PreRelease {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl PreRelease {
    /// Execute `pre-release` subcommand workflow.
    #[tracing::instrument(name = "pre-release", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {} = self;

        let _ = config; // supress unused-variables warning

        #[cfg(feature = "subcommand-lint")]
        super::Lint {
            feature_args: crate::args::FeatureArgs::EXHAUSTIVE,
        }
        .run(config)?;

        #[cfg(feature = "subcommand-test")]
        super::Test {
            feature_args: crate::args::FeatureArgs::EXHAUSTIVE,
            extra_options: vec![],
        }
        .run(config)?;

        Ok(())
    }
}
