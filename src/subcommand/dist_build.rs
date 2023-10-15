use crate::{config::Config, Result, Run};

/// Arguments definition of the `dist-build` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-dist-build.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct DistBuild {
    /// `dist-build-bin` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
    #[clap(flatten)]
    pub dist_build_bin_args: super::DistBuildBin,

    /// `dist-build-completion` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-completion")))]
    #[clap(flatten)]
    pub dist_build_completion_args: super::DistBuildCompletion,

    /// `dist-build-doc` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
    #[clap(flatten)]
    pub dist_build_doc_args: super::DistBuildDoc,

    /// `dist-build-license` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
    #[clap(flatten)]
    pub dist_build_license_args: super::DistBuildLicense,

    /// `dist-build-man` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-man")))]
    #[clap(flatten)]
    pub dist_build_man_args: super::DistBuildMan,

    /// `dist-build-readme` subcommand arguments.
    #[cfg(feature = "subcommand-dist-build-readme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-readme")))]
    #[clap(flatten)]
    pub dist_build_readme_args: super::DistBuildReadme,
}

impl Run for DistBuild {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl DistBuild {
    /// Runs the `dist-build` subcommand.
    #[tracing::instrument(name = "dist-build", skip_all, err)]
    pub fn run(&self, config: &Config) -> Result<()> {
        let Self {
            #[cfg(feature = "subcommand-dist-build-bin")]
            dist_build_bin_args,
            #[cfg(feature = "subcommand-dist-build-completion")]
            dist_build_completion_args,
            #[cfg(feature = "subcommand-dist-build-doc")]
            dist_build_doc_args,
            #[cfg(feature = "subcommand-dist-build-license")]
            dist_build_license_args,
            #[cfg(feature = "subcommand-dist-build-man")]
            dist_build_man_args,
            #[cfg(feature = "subcommand-dist-build-readme")]
            dist_build_readme_args,
        } = self;

        #[cfg(feature = "subcommand-dist-build-bin")]
        dist_build_bin_args.run(config)?;

        #[cfg(feature = "subcommand-dist-build-completion")]
        dist_build_completion_args.run(config)?;

        #[cfg(feature = "subcommand-dist-build-doc")]
        dist_build_doc_args.run(config)?;

        #[cfg(feature = "subcommand-dist-build-license")]
        dist_build_license_args.run(config)?;

        #[cfg(feature = "subcommand-dist-build-man")]
        dist_build_man_args.run(config)?;

        #[cfg(feature = "subcommand-dist-build-readme")]
        dist_build_readme_args.run(config)?;

        Ok(())
    }
}
