use crate::DistConfig;

feature_command_build! {
    mod build;
    pub use build::Build;
}

feature_command_build_bin! {
    mod build_bin;
    pub use build_bin::BuildBin;
}

feature_command_build_completion! {
    mod build_completion;
    pub use build_completion::BuildCompletion;
}

feature_command_build_doc! {
    mod build_doc;
    pub use build_doc::BuildDoc;
}

feature_command_build_license! {
    mod build_license;
    pub use build_license::BuildLicense;
}

feature_command_build_man! {
    mod build_man;
    pub use build_man::BuildMan;
}

feature_command_dist! {
    mod dist;
    pub use dist::Dist;
}

/// `xtask` command arguments.
#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Build all artifacts
    #[cfg(command_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-*")))]
    Build(Build),

    /// Build the release binaries
    #[cfg(feature = "command-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-bin")))]
    BuildBin(BuildBin),

    /// Build the shell completion files
    #[cfg(feature = "command-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-completion")))]
    BuildCompletion(BuildCompletion),

    /// Build the documents
    #[cfg(feature = "command-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-doc")))]
    BuildDoc(BuildDoc),

    /// Build the license files
    #[cfg(feature = "command-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-license")))]
    BuildLicense(BuildLicense),

    /// Build the man pages
    #[cfg(feature = "command-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build-man")))]
    BuildMan(BuildMan),

    /// Crate the archive for distribution
    #[cfg(feature = "command-dist")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
    Dist(Dist),
}

impl Command {
    /// Execute subcommand workflow.
    pub fn run(&self, config: &DistConfig) -> eyre::Result<()> {
        match self {
            #[cfg(command_build)]
            Self::Build(args) => args.run(config),

            #[cfg(feature = "command-build-bin")]
            Self::BuildBin(args) => args.run(config),

            #[cfg(feature = "command-build-completion")]
            Self::BuildCompletion(args) => args.run(config),

            #[cfg(feature = "command-build-doc")]
            Self::BuildDoc(args) => args.run(config),

            #[cfg(feature = "command-build-license")]
            Self::BuildLicense(args) => args.run(config),

            #[cfg(feature = "command-build-man")]
            Self::BuildMan(args) => args.run(config),

            #[cfg(feature = "command-dist")]
            Self::Dist(args) => args.run(config),
        }
    }
}
