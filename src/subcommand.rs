//! Command line interfaces for xtask workflows.

use crate::{config::Config, Result, Run};

#[cfg(feature = "subcommand-build")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-build")))]
mod build;
#[cfg(feature = "subcommand-build")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-build")))]
pub use self::build::Build;

#[cfg(feature = "subcommand-clippy")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-clippy")))]
mod clippy;
#[cfg(feature = "subcommand-clippy")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-clippy")))]
pub use self::clippy::Clippy;

#[cfg(feature = "subcommand-dist-archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-archive")))]
mod dist_archive;
#[cfg(feature = "subcommand-dist-archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-archive")))]
pub use self::dist_archive::DistArchive;

#[cfg(subcommand_dist_build)]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-*")))]
mod dist_build;
#[cfg(subcommand_dist_build)]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-*")))]
pub use self::dist_build::DistBuild;

#[cfg(feature = "subcommand-dist-build-bin")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
mod dist_build_bin;
#[cfg(feature = "subcommand-dist-build-bin")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
pub use self::dist_build_bin::DistBuildBin;

#[cfg(feature = "subcommand-dist-build-completion")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-completion")))]
mod dist_build_completion;
#[cfg(feature = "subcommand-dist-build-completion")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-completion")))]
pub use self::dist_build_completion::DistBuildCompletion;

#[cfg(feature = "subcommand-dist-build-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
mod dist_build_doc;
#[cfg(feature = "subcommand-dist-build-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
pub use self::dist_build_doc::DistBuildDoc;

#[cfg(feature = "subcommand-dist-build-license")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
mod dist_build_license;
#[cfg(feature = "subcommand-dist-build-license")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
pub use self::dist_build_license::DistBuildLicense;

#[cfg(feature = "subcommand-dist-build-man")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-man")))]
mod dist_build_man;
#[cfg(feature = "subcommand-dist-build-man")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-man")))]
pub use self::dist_build_man::DistBuildMan;

#[cfg(feature = "subcommand-dist-build-readme")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-readme")))]
mod dist_build_readme;
#[cfg(feature = "subcommand-dist-build-readme")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-readme")))]
pub use self::dist_build_readme::DistBuildReadme;

#[cfg(feature = "subcommand-dist-clean")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-clean")))]
mod dist_clean;
#[cfg(feature = "subcommand-dist-clean")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-clean")))]
pub use self::dist_clean::DistClean;

#[cfg(feature = "subcommand-dist")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist")))]
mod dist;
#[cfg(feature = "subcommand-dist")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist")))]
pub use self::dist::Dist;

#[cfg(feature = "subcommand-exec")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-exec")))]
mod exec;
#[cfg(feature = "subcommand-exec")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-exec")))]
pub use self::exec::Exec;

#[cfg(feature = "subcommand-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-doc")))]
mod doc;
#[cfg(feature = "subcommand-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-doc")))]
pub use self::doc::Doc;

#[cfg(feature = "subcommand-fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-fmt")))]
mod fmt;
#[cfg(feature = "subcommand-fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-fmt")))]
pub use self::fmt::Fmt;

#[cfg(feature = "subcommand-lint")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-lint")))]
mod lint;
#[cfg(feature = "subcommand-lint")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-lint")))]
pub use self::lint::Lint;

#[cfg(feature = "subcommand-pre-release")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-pre-release")))]
mod pre_release;
#[cfg(feature = "subcommand-pre-release")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-pre-release")))]
pub use self::pre_release::PreRelease;

#[cfg(feature = "subcommand-rdme")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-rdme")))]
mod rdme;
#[cfg(feature = "subcommand-rdme")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-rdme")))]
pub use self::rdme::Rdme;

#[cfg(feature = "subcommand-test")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-test")))]
mod test;
#[cfg(feature = "subcommand-test")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-test")))]
pub use self::test::Test;

#[cfg(feature = "subcommand-tidy")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-tidy")))]
mod tidy;
#[cfg(feature = "subcommand-tidy")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-tidy")))]
pub use self::tidy::Tidy;

#[cfg(feature = "subcommand-udeps")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-udeps")))]
mod udeps;
#[cfg(feature = "subcommand-udeps")]
#[cfg_attr(docsrs, doc(cfg(feature = "subcommand-udeps")))]
pub use self::udeps::Udeps;

/// Subcommand definition for cargo xtask command.
#[cfg_attr(doc, doc = include_str!("../doc/cargo-xtask.md"))]
///
/// # Examples
///
/// You can use this struct to define a subcommand of your application:
///
/// ```rust
/// use cli_xtask::{clap, config::Config, subcommand::Subcommand, Result};
///
/// #[derive(Debug, clap::Parser)]
/// struct App {
///     #[clap(subcommand)]
///     subcommand: Subcommand,
/// }
///
/// impl App {
///     pub fn run(&self, config: &Config) -> Result<()> {
///         self.subcommand.run(config)
///     }
/// }
/// ```
///
/// You can mix the subcommands defined in this enum with your own subcommands:
///
/// ```rust
/// use cli_xtask::{clap, config::Config, Result};
///
/// #[derive(Debug, clap::Parser)]
/// struct App {
///     #[clap(subcommand)]
///     subcommand: YourOwnSubcommand,
/// }
///
/// #[derive(Debug, clap::Subcommand)]
/// enum YourOwnSubcommand {
///     #[clap(flatten)]
///     Predefined(cli_xtask::subcommand::Subcommand),
///     Foo,
///     Bar,
/// }
/// ```
#[derive(Debug, clap::Subcommand)]
#[clap(bin_name = "cargo xtask", about = "Rust project automation command", long_about = None)]
#[non_exhaustive]
pub enum Subcommand {
    /// `cargo build` with options useful for testing and continuous
    /// integration.
    #[cfg(feature = "subcommand-build")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-build")))]
    Build(Build),

    /// `cargo clippy` with options useful for tesing and continuous
    /// integration.
    #[cfg(feature = "subcommand-clippy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-clippy")))]
    Clippy(Clippy),

    /// Build the artifacs and create the archive file for distribution.
    #[cfg(feature = "subcommand-dist")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist")))]
    Dist(Dist),

    /// Create the archive file for distribution.
    #[cfg(feature = "subcommand-dist-archive")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-archive")))]
    DistArchive(DistArchive),

    /// Build all artifacts for distribution.
    #[cfg(subcommand_dist_build)]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-*")))]
    DistBuild(DistBuild),

    /// Build the release binaries for distribution.
    #[cfg(feature = "subcommand-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
    DistBuildBin(DistBuildBin),

    /// Build the shell completion files for distribution.
    #[cfg(feature = "subcommand-dist-build-completion")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-completion")))]
    DistBuildCompletion(DistBuildCompletion),

    /// Build the documentation for distribution.
    #[cfg(feature = "subcommand-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
    DistBuildDoc(DistBuildDoc),

    /// Build the license files for distribution.
    #[cfg(feature = "subcommand-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
    DistBuildLicense(DistBuildLicense),

    /// Build the man pages for distribution.
    #[cfg(feature = "subcommand-dist-build-man")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-man")))]
    DistBuildMan(DistBuildMan),

    /// Build the readme files for distribution.
    #[cfg(feature = "subcommand-dist-build-readme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-readme")))]
    DistBuildReadme(DistBuildReadme),

    /// Remove the artifacts and archives for distribution.
    #[cfg(feature = "subcommand-dist-clean")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-clean")))]
    DistClean(DistClean),

    /// `cargo doc` with options useful for testing and continuous integration.
    #[cfg(feature = "subcommand-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-doc")))]
    Doc(Doc),

    /// Run commands on all workspaces in the current directory and
    /// subdirectories.
    #[cfg(feature = "subcommand-exec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-exec")))]
    Exec(Exec),

    /// `cargo fmt` with options useful for testing and continuous integration.
    #[cfg(feature = "subcommand-fmt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-fmt")))]
    Fmt(Fmt),

    /// Run lint commands at once.
    #[cfg(feature = "subcommand-lint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-lint")))]
    Lint(Lint),

    /// Run pre-release checks.
    #[cfg(feature = "subcommand-pre-release")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-pre-release")))]
    PreRelease(PreRelease),

    /// `cargo rdme` with options useful for testing and continuous integration.
    #[cfg(feature = "subcommand-rdme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-rdme")))]
    Rdme(rdme::Rdme),

    /// `cargo test` with options useful for testing and continuous integration.
    #[cfg(feature = "subcommand-test")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-test")))]
    Test(Test),

    /// Fix the package problems.
    #[cfg(feature = "subcommand-tidy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-tidy")))]
    Tidy(Tidy),

    /// `cargo udeps` with options useful for testing and continuous
    /// integration.
    #[cfg(feature = "subcommand-udeps")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-udeps")))]
    Udeps(Udeps),
}

impl Run for Subcommand {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Subcommand {
    /// Runs the subcommand specified by the command line arguments.
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            #[cfg(feature = "subcommand-build")]
            Self::Build(args) => args.run(config),

            #[cfg(feature = "subcommand-clippy")]
            Self::Clippy(args) => args.run(config),

            #[cfg(feature = "subcommand-dist")]
            Self::Dist(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-archive")]
            Self::DistArchive(args) => args.run(config),

            #[cfg(subcommand_dist_build)]
            Self::DistBuild(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-bin")]
            Self::DistBuildBin(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-completion")]
            Self::DistBuildCompletion(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-doc")]
            Self::DistBuildDoc(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-license")]
            Self::DistBuildLicense(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-man")]
            Self::DistBuildMan(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-build-readme")]
            Self::DistBuildReadme(args) => args.run(config),

            #[cfg(feature = "subcommand-dist-clean")]
            Self::DistClean(args) => args.run(config),

            #[cfg(feature = "subcommand-doc")]
            Self::Doc(args) => args.run(config),

            #[cfg(feature = "subcommand-exec")]
            Self::Exec(args) => args.run(config),

            #[cfg(feature = "subcommand-fmt")]
            Self::Fmt(args) => args.run(config),

            #[cfg(feature = "subcommand-lint")]
            Self::Lint(args) => args.run(config),

            #[cfg(feature = "subcommand-pre-release")]
            Self::PreRelease(args) => args.run(config),

            #[cfg(feature = "subcommand-rdme")]
            Self::Rdme(args) => args.run(config),

            #[cfg(feature = "subcommand-test")]
            Self::Test(args) => args.run(config),

            #[cfg(feature = "subcommand-tidy")]
            Self::Tidy(args) => args.run(config),

            #[cfg(feature = "subcommand-udeps")]
            Self::Udeps(args) => args.run(config),

            #[cfg(not(subcommand))]
            _ => unreachable!("no commands defined: {config:?}"),
        }
    }
}
