use crate::Config;

feature_command_dist_archive! {
    mod dist_archive;
    pub use dist_archive::DistArchive;
}

feature_command_dist_build! {
    mod dist_build;
    pub use dist_build::DistBuild;
}

feature_command_dist_build_bin! {
    mod dist_build_bin;
    pub use dist_build_bin::DistBuildBin;
}

feature_command_dist_build_completion! {
    mod dist_build_completion;
    pub use dist_build_completion::DistBuildCompletion;
}

feature_command_dist_build_doc! {
    mod dist_build_doc;
    pub use dist_build_doc::DistBuildDoc;
}

feature_command_dist_build_license! {
    mod dist_build_license;
    pub use dist_build_license::DistBuildLicense;
}

feature_command_dist_build_man! {
    mod dist_build_man;
    pub use dist_build_man::DistBuildMan;
}

feature_command_dist_build_readme! {
    mod dist_build_readme;
    pub use dist_build_readme::DistBuildReadme;
}

feature_command_dist_clean! {
    mod dist_clean;
    pub use dist_clean::DistClean;
}

feature_command_dist! {
    mod dist;
    pub use dist::Dist;
}

/// `xtask` command arguments.
#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Create the archive file for distribution
    #[cfg(feature = "command-dist-archive")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-archive")))]
    DistArchive(DistArchive),

    /// Build all artifacts for distribution
    #[cfg(command_dist_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
    DistBuild(DistBuild),

    /// Build the release binaries dor distribution
    #[cfg(feature = "command-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-bin")))]
    DistBuildBin(DistBuildBin),

    /// Build the shell completion files for distribution
    #[cfg(feature = "command-dist-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-completion")))]
    DistBuildCompletion(DistBuildCompletion),

    /// Build the documentation for distribution
    #[cfg(feature = "command-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-doc")))]
    DistBuildDoc(DistBuildDoc),

    /// Build the license files for distribution
    #[cfg(feature = "command-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-license")))]
    DistBuildLicense(DistBuildLicense),

    /// Build the man pages for distribution
    #[cfg(feature = "command-dist-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-man")))]
    DistBuildMan(DistBuildMan),

    /// Build the readme files for distribution
    #[cfg(feature = "command-dist-build-readme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-readme")))]
    DistBuildReadme(DistBuildReadme),

    /// Removes the dist artifacts
    #[cfg(feature = "command-dist-clean")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-clean")))]
    DistClean(DistClean),

    /// Crate the archive file for distribution
    #[cfg(feature = "command-dist")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
    Dist(Dist),
}

impl Command {
    /// Execute subcommand workflow.
    pub fn run(&self, config: &Config) -> eyre::Result<()> {
        match self {
            #[cfg(feature = "command-dist-archive")]
            Self::DistArchive(args) => args.run(config.dist()?),

            #[cfg(command_dist_build)]
            Self::DistBuild(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-bin")]
            Self::DistBuildBin(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-completion")]
            Self::DistBuildCompletion(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-doc")]
            Self::DistBuildDoc(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-license")]
            Self::DistBuildLicense(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-man")]
            Self::DistBuildMan(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-build-readme")]
            Self::DistBuildReadme(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist-clean")]
            Self::DistClean(args) => args.run(config.dist()?),

            #[cfg(feature = "command-dist")]
            Self::Dist(args) => args.run(config.dist()?),
        }
    }
}
