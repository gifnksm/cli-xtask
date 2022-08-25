use crate::{args::FeatureArgs, config::Config, Result, Run};

/// Arguments definition of the `tidy` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-tidy.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Tidy {
    /// Fix code even if a VCS was not detected.
    #[clap(long)]
    allow_no_vcs: bool,
    /// Fix code even if the working directory is dirty.
    #[clap(long)]
    allow_dirty: bool,
    /// Fix code even if the working directory has staged changes.
    #[clap(long)]
    allow_staged: bool,
    /// Features to run the cargo commands with.
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
}

impl Run for Tidy {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Tidy {
    /// Returns a list of all subcommands to run.
    pub fn subcommands(&self) -> Vec<Box<dyn Run>> {
        let Self {
            allow_no_vcs,
            allow_dirty,
            allow_staged,
            feature_args,
        } = self;

        let _ = allow_no_vcs; // supress unused-variables warning
        let _ = allow_dirty; // supress unused-variables warning
        let _ = allow_staged; // supress unused-variables warning
        let _ = feature_args.clone(); // supress unused-variables warning

        vec![
            // cargo fmt
            #[cfg(feature = "subcommand-fmt")]
            Box::new(super::Fmt {
                env_args: Default::default(),
                package_args: feature_args.package_args.clone(),
                extra_options: vec![],
            }),
            // cargo clippy --fix
            #[cfg(feature = "subcommand-clippy")]
            Box::new({
                let mut clippy_options = vec![];
                if *allow_no_vcs {
                    clippy_options.push("--allow-no-vcs");
                }
                if *allow_dirty {
                    clippy_options.push("--allow-dirty");
                }
                if *allow_staged {
                    clippy_options.push("--allow-staged");
                }

                super::Clippy {
                    env_args: Default::default(),
                    feature_args: feature_args.clone(),
                    extra_options: ["--fix", "--all-targets"]
                        .into_iter()
                        .chain(clippy_options)
                        .map(String::from)
                        .collect(),
                }
            }),
            // cargo rdme
            #[cfg(feature = "subcommand-rdme")]
            Box::new({
                let mut rdme_options = vec![];
                if *allow_dirty || *allow_staged {
                    rdme_options.push("--force");
                }
                super::Rdme {
                    env_args: Default::default(),
                    workspace_args: feature_args.package_args.workspace_args.clone(),
                    extra_options: rdme_options.into_iter().map(String::from).collect(),
                }
            }),
        ]
    }

    /// Runs the `tidy` subcommand.
    #[tracing::instrument(name = "tidy", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        for subcommand in self.subcommands() {
            subcommand.run(config)?;
        }
        Ok(())
    }
}
