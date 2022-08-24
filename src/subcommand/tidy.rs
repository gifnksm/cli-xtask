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
    /// Runs the `tidy` subcommand.
    #[tracing::instrument(name = "tidy", parent = None, skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {
            allow_no_vcs,
            allow_dirty,
            allow_staged,
            feature_args,
        } = self;

        let _ = config; // supress unused-variables warning
        let _ = allow_no_vcs; // supress unused-variables warning
        let _ = allow_dirty; // supress unused-variables warning
        let _ = allow_staged; // supress unused-variables warning
        let _ = feature_args.clone(); // supress unused-variables warning

        // cargo fmt
        #[cfg(feature = "subcommand-fmt")]
        super::Fmt {
            env_args: Default::default(),
            package_args: feature_args.package_args.clone(),
            extra_options: vec![],
        }
        .run(config)?;

        #[cfg(feature = "subcommand-clippy")]
        {
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

            // cargo clippy --fix
            super::Clippy {
                env_args: Default::default(),
                feature_args: feature_args.clone(),
                extra_options: ["--fix", "--all-targets"]
                    .into_iter()
                    .chain(clippy_options)
                    .map(String::from)
                    .collect(),
            }
            .run(config)?;
        }

        #[cfg(feature = "subcommand-rdme")]
        {
            let mut rdme_options = vec![];
            if *allow_dirty || *allow_staged {
                rdme_options.push("--force");
            }
            // cargo rdme
            super::Rdme {
                env_args: Default::default(),
                workspace_args: feature_args.package_args.workspace_args.clone(),
                extra_options: rdme_options.into_iter().map(String::from).collect(),
            }
            .run(config)?;
        }

        Ok(())
    }
}
