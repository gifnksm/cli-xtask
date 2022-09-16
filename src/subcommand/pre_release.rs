use std::any::Any;

use crate::{config::Config, Result, Run, SubcommandRun};

/// Arguments definition of the `pre-release` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-pre-release.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct PreRelease {}

impl Run for PreRelease {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }

    fn to_subcommands(&self) -> Option<SubcommandRun> {
        Some(self.to_subcommands())
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

    /// Returns the subcommands that this command will run.
    pub fn to_subcommands(&self) -> SubcommandRun {
        SubcommandRun::new(self.subcommands())
    }

    /// Runs the `pre-release` subcommand.
    #[tracing::instrument(name = "pre-release", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        self.to_subcommands().run(config)
    }
}
