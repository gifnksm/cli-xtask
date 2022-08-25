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
    /// Returns a list of all subcommands to run.
    pub fn subcommands(&self) -> Vec<Box<dyn Run>> {
        let Self { feature_args } = self;
        let _ = feature_args.clone(); // supress unused-variables warning

        vec![
            // cargo fmt --check
            #[cfg(feature = "subcommand-fmt")]
            Box::new(super::Fmt {
                env_args: Default::default(),
                package_args: feature_args.package_args.clone(),
                extra_options: ["--check"].into_iter().map(String::from).collect(),
            }),
            // cargo clippy -- -D warnings
            #[cfg(feature = "subcommand-clippy")]
            Box::new(super::Clippy {
                env_args: Default::default(),
                feature_args: feature_args.clone(),
                extra_options: ["--all-targets", "--", "-D", "warnings"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            }),
            // RUSTDOCFLAGS="-D warnings" cargo doc
            #[cfg(feature = "subcommand-doc")]
            Box::new(super::Doc {
                env_args: crate::args::EnvArgs::new([("RUSTDOCFLAGS", "-D warnings")]),
                package_args: feature_args.package_args.clone(),
                extra_options: ["--no-deps"].into_iter().map(String::from).collect(),
            }),
            #[cfg(feature = "subcommand-docsrs")]
            Box::new(super::Docsrs {
                env_args: crate::args::EnvArgs::new([("RUSTDOCFLAGS", "-D warnings")]),
                package_args: feature_args.package_args.clone(),
                default_target: false,
                all_targets: false,
                extra_options: vec![],
            }),
            // cargo rdme --check
            #[cfg(feature = "subcommand-rdme")]
            Box::new(super::Rdme {
                env_args: Default::default(),
                workspace_args: feature_args.package_args.workspace_args.clone(),
                extra_options: ["--check"].into_iter().map(String::from).collect(),
            }),
            // cargo udeps
            #[cfg(feature = "subcommand-udeps")]
            Box::new(super::Udeps {
                env_args: Default::default(),
                feature_args: feature_args.clone(),
                extra_options: vec![],
            }),
        ]
    }

    /// Runs the `lint` subcommand.
    #[tracing::instrument(name = "lint", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        for subcommand in self.subcommands() {
            subcommand.run(config)?;
        }
        Ok(())
    }
}
