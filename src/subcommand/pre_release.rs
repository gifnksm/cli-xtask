use crate::{config::Config, Result, Run};

/// Arguments definition of the `pre-release` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-pre-release.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct PreRelease {}

impl Run for PreRelease {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl PreRelease {
    /// Returns a list of all subcommands to run.
    pub fn subcommands(&self) -> Vec<Box<dyn Run>> {
        let Self {} = self;
        vec![
            #[cfg(feature = "subcommand-lint")]
            Box::new(super::Lint {
                feature_args: crate::args::FeatureArgs::EXHAUSTIVE,
            }),
            #[cfg(feature = "subcommand-test")]
            Box::new(super::Test {
                env_args: Default::default(),
                feature_args: crate::args::FeatureArgs::EXHAUSTIVE,
                extra_options: vec![],
            }),
        ]
    }

    /// Runs the `pre-release` subcommand.
    #[tracing::instrument(name = "pre-release", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        for subcommand in self.subcommands() {
            subcommand.run(config)?;
        }
        Ok(())
    }
}
