use crate::{args::FeatureArgs, config::Config, Result, Run};

/// Arguments definition of the `lint` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-lint.md"))]
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
    /// Runs the `lint` subcommand.
    #[tracing::instrument(name = "lint", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self { feature_args } = self;

        let _ = config; // supress unused-variables warning
        let _ = feature_args.clone(); // supress unused-variables warning

        // cargo fmt --check
        #[cfg(feature = "subcommand-fmt")]
        super::Fmt {
            env_args: Default::default(),
            package_args: feature_args.package_args.clone(),
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;

        // cargo clippy -- -D warnings
        #[cfg(feature = "subcommand-clippy")]
        super::Clippy {
            env_args: Default::default(),
            feature_args: feature_args.clone(),
            extra_options: ["--all-targets", "--", "-D", "warnings"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
        .run(config)?;

        // RUSTDOCFLAGS="-D warnings" cargo doc
        #[cfg(feature = "subcommand-doc")]
        super::Doc {
            env_args: crate::args::EnvArgs::new([("RUSTDOCFLAGS", "-D warnings")]),
            package_args: feature_args.package_args.clone(),
            extra_options: ["--no-deps"].into_iter().map(String::from).collect(),
        }
        .run(config)?;

        // cargo rdme --check
        #[cfg(feature = "subcommand-rdme")]
        super::Rdme {
            env_args: Default::default(),
            workspace_args: feature_args.package_args.workspace_args.clone(),
            extra_options: ["--check"].into_iter().map(String::from).collect(),
        }
        .run(config)?;

        // cargo udeps
        #[cfg(feature = "subcommand-udeps")]
        super::Udeps {
            env_args: Default::default(),
            feature_args: feature_args.clone(),
            extra_options: vec![],
        }
        .run(config)?;

        Ok(())
    }
}
