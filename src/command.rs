//! Command line interfaces for xtask workflows.

use crate::{config::Config, Result, Run};

#[cfg(feature = "command-build")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-build")))]
mod build;
#[cfg(feature = "command-build")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-build")))]
pub use self::build::Build;

#[cfg(feature = "command-clippy")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-clippy")))]
mod clippy;
#[cfg(feature = "command-clippy")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-clippy")))]
pub use self::clippy::Clippy;

#[cfg(feature = "command-dist-archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-archive")))]
mod dist_archive;
#[cfg(feature = "command-dist-archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-archive")))]
pub use self::dist_archive::DistArchive;

#[cfg(command_dist_build)]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
mod dist_build;
#[cfg(command_dist_build)]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-*")))]
pub use self::dist_build::DistBuild;

#[cfg(feature = "command-dist-build-bin")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-bin")))]
mod dist_build_bin;
#[cfg(feature = "command-dist-build-bin")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-bin")))]
pub use self::dist_build_bin::DistBuildBin;

#[cfg(feature = "command-dist-build-completion")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-completion")))]
mod dist_build_completion;
#[cfg(feature = "command-dist-build-completion")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-completion")))]
pub use self::dist_build_completion::DistBuildCompletion;

#[cfg(feature = "command-dist-build-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-doc")))]
mod dist_build_doc;
#[cfg(feature = "command-dist-build-doc")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-doc")))]
pub use self::dist_build_doc::DistBuildDoc;

#[cfg(feature = "command-dist-build-license")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-license")))]
mod dist_build_license;
#[cfg(feature = "command-dist-build-license")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-license")))]
pub use self::dist_build_license::DistBuildLicense;

#[cfg(feature = "command-dist-build-man")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-man")))]
mod dist_build_man;
#[cfg(feature = "command-dist-build-man")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-man")))]
pub use self::dist_build_man::DistBuildMan;

#[cfg(feature = "command-dist-build-readme")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-readme")))]
mod dist_build_readme;
#[cfg(feature = "command-dist-build-readme")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-build-readme")))]
pub use self::dist_build_readme::DistBuildReadme;

#[cfg(feature = "command-dist-clean")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-clean")))]
mod dist_clean;
#[cfg(feature = "command-dist-clean")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist-clean")))]
pub use self::dist_clean::DistClean;

#[cfg(feature = "command-dist")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
mod dist;
#[cfg(feature = "command-dist")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
pub use self::dist::Dist;

#[cfg(feature = "command-exec")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-exec")))]
mod exec;
#[cfg(feature = "command-exec")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-exec")))]
pub use self::exec::Exec;

#[cfg(feature = "command-fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-fmt")))]
mod fmt;
#[cfg(feature = "command-fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-fmt")))]
pub use self::fmt::Fmt;

#[cfg(feature = "command-lint")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-lint")))]
mod lint;
#[cfg(feature = "command-lint")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-lint")))]
pub use self::lint::Lint;

#[cfg(feature = "command-rdme")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-rdme")))]
mod rdme;
#[cfg(feature = "command-rdme")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-rdme")))]
pub use self::rdme::Rdme;

#[cfg(feature = "command-test")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-test")))]
mod test;
#[cfg(feature = "command-test")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-test")))]
pub use self::test::Test;

#[cfg(feature = "command-udeps")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-udeps")))]
mod udeps;
#[cfg(feature = "command-udeps")]
#[cfg_attr(docsrs, doc(cfg(feature = "command-udeps")))]
pub use self::udeps::Udeps;

/// Subcommand definition.
///
/// # Examples
///
/// You can use this struct to define a subcommand of your application:
///
/// ```rust
/// use cli_xtask::{clap, command::Command, config::Config, Result};
///
/// #[derive(Debug, clap::Parser)]
/// struct Args {
///     #[clap(subcommand)]
///     command: Command,
/// }
///
/// impl Args {
///     pub fn run(&self, config: &Config) -> Result<()> {
///         self.command.run(config)
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
/// struct Args {
///     #[clap(subcommand)]
///     command: Command,
/// }
///
/// #[derive(Debug, clap::Subcommand)]
/// enum Command {
///     #[clap(flatten)]
///     Command(cli_xtask::command::Command),
///     YourOwnCommand(YourOwnCommand),
/// }
///
/// #[derive(Debug, clap::Args)]
/// struct YourOwnCommand {}
/// ```
#[derive(Debug, clap::Subcommand)]
#[clap(bin_name = "cargo xtask", about = "Rust project automation command", long_about = None)]
#[non_exhaustive]
pub enum Command {
    /// Run `cargo build` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-build")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-build")))]
    Build(Build),

    /// Run `cargo clippy` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-clippy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-clippy")))]
    Clippy(Clippy),

    /// Build the artifacs and create the archive file for distribution
    #[cfg(feature = "command-dist")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist")))]
    Dist(Dist),

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

    /// Remove the artifacts and archives for distribution
    #[cfg(feature = "command-dist-clean")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-dist-clean")))]
    DistClean(DistClean),

    /// Run commands on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-exec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-exec")))]
    Exec(Exec),

    /// Run `cargo fmt` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-fmt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-fmt")))]
    Fmt(Fmt),

    /// Run all lint commands on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-lint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-lint")))]
    Lint(Lint),

    /// Run `cargo rdme` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-rdme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-rdme")))]
    Rdme(rdme::Rdme),

    /// Run `cargo test` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-test")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-test")))]
    Test(Test),

    /// Run `cargo udeps` on all workspaces in the current directory and
    /// subdirectories
    #[cfg(feature = "command-udeps")]
    #[cfg_attr(docsrs, doc(cfg(feature = "command-udeps")))]
    Udeps(Udeps),
}

impl Run for Command {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Command {
    /// Runs subcommand workflow.
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            #[cfg(feature = "command-build")]
            Self::Build(args) => args.run(config),

            #[cfg(feature = "command-clippy")]
            Self::Clippy(args) => args.run(config),

            #[cfg(feature = "command-dist")]
            Self::Dist(args) => args.run(config),

            #[cfg(feature = "command-dist-archive")]
            Self::DistArchive(args) => args.run(config),

            #[cfg(command_dist_build)]
            Self::DistBuild(args) => args.run(config),

            #[cfg(feature = "command-dist-build-bin")]
            Self::DistBuildBin(args) => args.run(config),

            #[cfg(feature = "command-dist-build-completion")]
            Self::DistBuildCompletion(args) => args.run(config),

            #[cfg(feature = "command-dist-build-doc")]
            Self::DistBuildDoc(args) => args.run(config),

            #[cfg(feature = "command-dist-build-license")]
            Self::DistBuildLicense(args) => args.run(config),

            #[cfg(feature = "command-dist-build-man")]
            Self::DistBuildMan(args) => args.run(config),

            #[cfg(feature = "command-dist-build-readme")]
            Self::DistBuildReadme(args) => args.run(config),

            #[cfg(feature = "command-dist-clean")]
            Self::DistClean(args) => args.run(config),

            #[cfg(feature = "command-exec")]
            Self::Exec(args) => args.run(config),

            #[cfg(feature = "command-fmt")]
            Self::Fmt(args) => args.run(config),

            #[cfg(feature = "command-lint")]
            Self::Lint(args) => args.run(config),

            #[cfg(feature = "command-rdme")]
            Self::Rdme(args) => args.run(config),

            #[cfg(feature = "command-test")]
            Self::Test(args) => args.run(config),

            #[cfg(feature = "command-udeps")]
            Self::Udeps(args) => args.run(config),

            #[cfg(not(command))]
            _ => unreachable!("no commands defined: {config:?}"),
        }
    }
}
