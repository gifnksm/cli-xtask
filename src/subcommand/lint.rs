use crate::{args::FeatureArgs, config::Config, Result, Run};

/// `lint` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Lint {
    /// Features to run the cargo commands with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
}

impl Run for Lint {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Lint {
    /// Execute `lint` subcommand workflow.
    #[tracing::instrument(name = "lint", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self { feature_args } = self;

        let _ = config; // supress unused-variables warning
        let _feature_args = feature_args.clone(); // supress unused-variables warning

        // cargo fmt --check
        #[cfg(feature = "subcommand-fmt")]
        super::Fmt {
            package_args: feature_args.package_args.clone(),
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;

        // cargo clippy -- -D warnings
        #[cfg(feature = "subcommand-clippy")]
        super::Clippy {
            feature_args: feature_args.clone(),
            extra_options: ["--all-targets", "--", "-D", "warnings"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
        .run(config)?;

        // cargo rdme --check
        #[cfg(feature = "subcommand-rdme")]
        super::Rdme {
            workspace_args: feature_args.package_args.workspace_args.clone(),
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;

        // cargo udeps
        #[cfg(feature = "subcommand-udeps")]
        super::Udeps {
            feature_args: feature_args.clone(),
            extra_options: vec![],
        }
        .run(config)?;

        Ok(())
    }
}
