use clap::Parser;

use crate::DistConfig;

/// `dbuild` subcommand arguments.
#[derive(Debug, Parser)]
pub struct DistBuild {
    /// `dist-build-bin` subcommand arguments.
    #[cfg(feature = "command-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-bin")))]
    #[clap(flatten)]
    pub dist_build_bin_args: super::DistBuildBin,

    /// `dist-build-completion` subcommand arguments.
    #[cfg(feature = "command-dist-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-completion")))]
    #[clap(flatten)]
    pub dist_build_completion_args: super::DistBuildCompletion,

    /// `dist-build-doc` subcommand arguments.
    #[cfg(feature = "command-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-doc")))]
    #[clap(flatten)]
    pub dist_build_doc_args: super::DistBuildDoc,

    /// `dist-build-license` subcommand arguments.
    #[cfg(feature = "command-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-license")))]
    #[clap(flatten)]
    pub dist_build_license_args: super::DistBuildLicense,

    /// `dist-build-man` subcommand arguments.
    #[cfg(feature = "command-dist-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-man")))]
    #[clap(flatten)]
    pub dist_build_man_args: super::DistBuildMan,
}

impl DistBuild {
    /// Execute `dist-build` subcommand workflow.
    #[tracing::instrument(name = "dist-build", parent = None, skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        let Self {
            #[cfg(feature = "command-dist-build-bin")]
            dist_build_bin_args,
            #[cfg(feature = "command-dist-build-completion")]
            dist_build_completion_args,
            #[cfg(feature = "command-dist-build-doc")]
            dist_build_doc_args,
            #[cfg(feature = "command-dist-build-license")]
            dist_build_license_args,
            #[cfg(feature = "command-dist-build-man")]
            dist_build_man_args,
        } = self;

        #[cfg(feature = "command-dist-build-bin")]
        dist_build_bin_args.run(config)?;

        #[cfg(feature = "command-dist-build-completion")]
        dist_build_completion_args.run(config)?;

        #[cfg(feature = "command-dist-build-doc")]
        dist_build_doc_args.run(config)?;

        #[cfg(feature = "command-dist-build-license")]
        dist_build_license_args.run(config)?;

        #[cfg(feature = "command-dist-build-man")]
        dist_build_man_args.run(config)?;

        Ok(())
    }
}
