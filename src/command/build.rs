use clap::Parser;

use crate::DistConfig;

/// `build` subcommand arguments.
#[derive(Debug, Parser)]
pub struct Build {
    /// `build-bin` subcommand arguments.
    #[cfg(feature = "command-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-bin")))]
    #[clap(flatten)]
    pub build_bin_args: super::BuildBin,

    /// `build-completion` subcommand arguments.
    #[cfg(feature = "command-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-completion")))]
    #[clap(flatten)]
    pub build_completion_args: super::BuildCompletion,

    /// `build-doc` subcommand arguments.
    #[cfg(feature = "command-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-doc")))]
    #[clap(flatten)]
    pub build_doc_args: super::BuildDoc,

    /// `build-license` subcommand arguments.
    #[cfg(feature = "command-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-license")))]
    #[clap(flatten)]
    pub build_license_args: super::BuildLicense,

    /// `build-man` subcommand arguments.
    #[cfg(feature = "command-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-man")))]
    #[clap(flatten)]
    pub build_man_args: super::BuildMan,
}

impl Build {
    /// Execute `build` subcommand workflow.
    #[tracing::instrument(name = "build", skip_all, err)]
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        let Self {
            #[cfg(feature = "command-build-bin")]
            build_bin_args,
            #[cfg(feature = "command-build-completion")]
            build_completion_args,
            #[cfg(feature = "command-build-doc")]
            build_doc_args,
            #[cfg(feature = "command-build-license")]
            build_license_args,
            #[cfg(feature = "command-build-man")]
            build_man_args,
        } = self;

        #[cfg(feature = "command-build-bin")]
        build_bin_args.run(config)?;

        #[cfg(feature = "command-build-completion")]
        build_completion_args.run(config)?;

        #[cfg(feature = "command-build-doc")]
        build_doc_args.run(config)?;

        #[cfg(feature = "command-build-license")]
        build_license_args.run(config)?;

        #[cfg(feature = "command-build-man")]
        build_man_args.run(config)?;

        Ok(())
    }
}
