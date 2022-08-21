use cli_xtask::{args::FeatureArgs, clap, config::Config, subcommand, tracing, Result};

/// `pre-release` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct PreRelease {}

impl PreRelease {
    /// Execute `xtask-test` subcommand workflow.
    #[tracing::instrument(name = "xtask-test", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {} = self;

        let mut lint = subcommand::Lint::default();
        lint.feature_args = FeatureArgs::EXHAUSTIVE;
        lint.run(config)?;

        let mut test = subcommand::Test::default();
        test.feature_args = FeatureArgs::EXHAUSTIVE;
        test.run(config)?;

        crate::xtask_test::XtaskTest {
            cargo_llvm_cov: false,
        }
        .run(config)?;

        Ok(())
    }
}
