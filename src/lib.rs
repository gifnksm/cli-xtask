#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A collection of utility functions and command line interfaces for
//! [cargo-xtask].
//!
//! This crate provides the following utilities:
//!
//! * **[`cargo xtask dist`]** and related subcommands
//!   * Builds a distributable tar.gz package for your bin crate.
//! * **[`cargo xtask lint`]** and related subcommands
//!   * Runs the lints for your bin/lib crate.
//!   * Integrated with [`rustdoc`], [`rustfmt`], [`clippy`],
//!     [`cargo-sync-rdme`], [`cargo-udeps`].
//! * **[`cargo xtask tidy`]** and related subcommands
//!   * Fixes the problems on your bin/lib crate.
//!   * Integrated with  [`rustfmt`], [`clippy`], [`cargo-sync-rdme`].
//! * **[`cargo xtask pre-release`]**
//!   * Checks if your bin/lib crate is ready for a release.
//! * **[`cargo xtask build`], [`clippy`][`cargo xtask clippy`], [`doc`][`cargo
//!   xtask doc`], [`fmt`][`cargo xtask fmt`], [`test`][`cargo xtask test`]**
//!   * Runs the cargo commands with options useful for testing and continuous
//!     integration.
//!     * **`--all-workspaces`** - Runs the cargo commands for all workspaces.
//!     * **`--workspace`** - Runs the cargo commands for all packages in the
//!       workspace.
//!     * **`--each-features`** - Repeats to runs the cargo commands for each
//!       feature enabled.
//!     * **`--exhaustive`** - Same as `--all-workspaces --workspace
//!       --each-features`.
//! * **[`cargo xtask docsrs`]**
//!   * Builds the documentation for your lib crate with configuration for
//!     [docs.rs].
//! * **[`cargo xtask exec`]**
//!   * Runs a command in the gicontext of all workspaces.
//!
//! # Usage
//!
//! First, create an `xtask` crate following the [instructions on the
//! cargo-xtask website][xtask-setup].
//!
//! Then, run the following command to add `cli-xtask` to the dependencies.
//!
//! * For bin crates:
//!
//!     ```console
//!     cargo add -p xtask cli-xtask --features main,bin-crate
//!     ```
//!
//!     If you want to use extra tools such as `cargo-sync-rdme` and
//! `cargo-udeps`,     add the `bin-crate-extra` feature.
//!
//!     ```console
//!     cargo add -p xtask cli-xtask --features main,bin-crate,bin-crate-extra
//!     ```
//!
//! * For lib crates:
//!
//!     ```console
//!     cargo add -p xtask cli-xtask --features main,lib-crate
//!     ```
//!
//!     If you want to use extra tools such as `cargo-sync-rdme` and
//! `cargo-udeps`,     add the `lib-crate-extra` feature.
//!
//!     ```console
//!     cargo add -p xtask cli-xtask --features main,lib-crate,lib-crate-extra
//!     ```
//!
//! Finally, edit `xtask/src/main.rs` as follows
//!
//! ```rust
//! # #[cfg(all(feature = "main"))]
//! # {
//! use cli_xtask::{Result, Xtask};
//!
//! fn main() -> Result<()> {
//!     <Xtask>::main()
//! }
//! # }
//! ```
//!
//! Now you can run various workflows with `cargo xtask`.
//!
//! [xtask-setup]: https://github.com/matklad/cargo-xtask#defining-xtasks
//!
//! # Customizing
//!
//! If you want to remove the subcommands that are not useful for your project,
//! you can remove them by disabling the corresponding cargo features.
//! See the [Feature flags section](#feature-flags) for more information.
//!
//! If you want to add the subcommands that are not included in this crate,
//! you can add them by creating a new data structure that implements the
//! [`clap::Subcommand`] and [`Run`].
//! See [the documentation of `Xtask`](Xtask) for more
//! information.
//!
//! # Feature flags
//!
//! By using the features flags of cli-xtask, you can enable only the features
//! and commands you need. By default, all features are disabled.
//!
//! The following section contains a list of available features:
//!
//! ## CLI features
//!
//! * **`main`** - Enables [`Xtask::main`] function and
//!   [`Xtask::main_with_config`] function that are the premade entry point for
//!   the CLI.
//! * **`error-handler`** - Enables functions for error handling in
//!   [`error_handler`] module.
//! * **`logger`** - Enables functions for logging in [`logger`] module.
//!
//! ## Subcommand features
//!
//! There are two types of features that enable subcommands:
//!
//! * **Combined features** - features that enable several useful subcommands at
//!   once, depending on the type of crate
//! * **Separated features** - features that enable each subcommand separately
//!
//! ### Combined features
//!
//! * **`bin-crate`**:- Enables useful subcommands for bin crates.
//! * **`lib-crate`** - Enables useful subcommands for lib crates.
//! * **`bin-crate-extra`** - Enables the additional subcommands useful for bin
//!   crates.
//! * **`lib-crate-extra`** - Enables the additional subcommands useful for lib
//!   crates.
//!
//! The `{bin,lib}-crate` feature requires only the standard Rust tools that can
//! be installed with `rustup`. The `{bin,lib}-crate-extra` feature may require
//! third-party tools.
//!
//! ### Separated features
//!
//! The following features require only the standard Rust tools:
//!
//! * **`subcommand-build`** - Enables [`cargo xtask build`].
//! * **`subcommand-clippy`** - Enables [`cargo xtask clippy`].
//! * **`subcommand-dist`** - Enables [`cargo xtask dist`].
//! * **`subcommand-dist-archive`** - Enables [`cargo xtask dist-archive`].
//! * **`subcommand-dist-build-bin`** - Enables [`cargo xtask dist-build-bin`].
//! * **`subcommand-dist-build-completion`** - Enables [`cargo xtask
//!   dist-build-completion`].
//! * **`subcommand-dist-build-doc`** - Enables [`cargo xtask dist-build-doc`].
//! * **`subcommand-dist-build-license`** - Enables [`cargo xtask
//!   dist-build-license`].
//! * **`subcommand-dist-build-man`** - Enables [`cargo xtask dist-build-man`].
//! * **`subcommand-dist-build-readme`** - Enables [`cargo xtask
//!   dist-build-readme`].
//! * **`subcommand-dist-clean`** - Enables [`cargo xtask dist-clean`].
//! * **`subcommand-doc`** - Enables [`cargo xtask doc`].
//! * **`subcommand-docsrs`** - Enables [`cargo xtask docsrs`].
//! * **`subcommand-exec`** - Enables [`cargo xtask exec`].
//! * **`subcommand-fmt`** - Enables [`cargo xtask fmt`].
//! * **`subcommand-lint`** - Enables [`cargo xtask lint`].
//! * **`subcommand-pre-release`** - Enables [`cargo xtask pre-release`].
//! * **`subcommand-test`** - Enables [`cargo xtask test`].
//! * **`subcommand-tidy`** - Enables [`cargo xtask tidy`].
//!
//! The following features require third-party tools:
//!
//! * **`subcommand-sync-rdme`** - Enables [`cargo xtask sync-rdme`]. Requires
//!   [`cargo-sync-rdme`] installed.
//! * **`subcommand-udeps`** - Enables [`cargo xtask udeps`]. Requires
//!   [`cargo-udeps`] installed.
//!
//! ## Other features
//!
//! * **`archive`** - Enables [`archive`] module which provides the
//!   functionality to create the archive file for distribution.
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The minimum supported Rust version is **Rust 1.62.0**.
//! At least the last 3 versions of stable Rust are supported at any given time.
//!
//! While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in
//! a patch release. Once a crate has reached 1.x, any MSRV bump will be
//! accompanied by a new minor version.
//!
//! # License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//! See [CONTRIBUTING.md].
//!
//! [cargo-xtask]: https://github.com/matklad/cargo-xtask
//! [`cargo xtask build`]: subcommand::Build
//! [`cargo xtask clippy`]: subcommand::Clippy
//! [`cargo xtask dist`]: subcommand::Dist
//! [`cargo xtask dist-archive`]: subcommand::DistArchive
//! [`cargo xtask dist-build-bin`]: subcommand::DistBuildBin
//! [`cargo xtask dist-build-completion`]: subcommand::DistBuildCompletion
//! [`cargo xtask dist-build-doc`]: subcommand::DistBuildDoc
//! [`cargo xtask dist-build-license`]: subcommand::DistBuildLicense
//! [`cargo xtask dist-build-man`]: subcommand::DistBuildMan
//! [`cargo xtask dist-build-readme`]: subcommand::DistBuildReadme
//! [`cargo xtask dist-clean`]: subcommand::DistClean
//! [`cargo xtask doc`]: subcommand::Doc
//! [`cargo xtask docsrs`]: subcommand::Docsrs
//! [`cargo xtask exec`]: subcommand::Exec
//! [`cargo xtask fmt`]: subcommand::Fmt
//! [`cargo xtask lint`]: subcommand::Lint
//! [`cargo xtask pre-release`]: subcommand::PreRelease
//! [`cargo xtask sync-rdme`]: subcommand::SyncRdme
//! [`cargo xtask test`]: subcommand::Test
//! [`cargo xtask tidy`]: subcommand::Tidy
//! [`cargo xtask udeps`]: subcommand::Udeps
//! [`rustdoc`]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
//! [`rustfmt`]: https://github.com/rust-lang/rustfmt
//! [`clippy`]: https://github.com/rust-lang/rust-clippy
//! [`cargo-sync-rdme`]: https://github.com/gifnksm/cargo-sync-rdme
//! [`cargo-udeps`]: https://github.com/est31/cargo-udeps
//! [docs.rs]: https://docs.rs/
//! [LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
//! [CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

#![doc(html_root_url = "https://docs.rs/cli-xtask/0.5.0")]

use std::{any::Any, fmt};

pub use cargo_metadata::{self, camino};
pub use clap;
#[cfg(feature = "error-handler")]
#[cfg_attr(docsrs, doc(cfg(feature = "error-handler")))]
pub use color_eyre;
pub use eyre;
pub use tracing;

#[cfg(feature = "error-handler")]
#[cfg_attr(docsrs, doc(cfg(feature = "error-handler")))]
pub mod error_handler;

#[cfg(feature = "logger")]
#[cfg_attr(docsrs, doc(cfg(feature = "logger")))]
pub use tracing_subscriber;

#[cfg(feature = "logger")]
#[cfg_attr(docsrs, doc(cfg(feature = "logger")))]
pub mod logger;

#[cfg(feature = "archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "archive")))]
pub mod archive;

pub mod args;
pub mod cargo;
mod command;
pub mod config;
pub mod fs;
pub mod process;
pub mod subcommand;
pub mod workspace;

pub use self::command::Xtask;

/// Error type for this crate.
pub type Error = eyre::Error;
/// Result type for this crate.
pub type Result<T> = eyre::Result<T>;

/// Runs the command or subcommand.
pub trait Run: Any {
    /// Runs the command or subcommand.
    fn run(&self, config: &config::Config) -> Result<()>;

    /// Returns the subcommands that this command will run.
    fn to_subcommands(&self) -> Option<SubcommandRun> {
        None
    }

    /// Converts the `Box<dyn Run>` to `Box<dyn Any>`.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    /// Converts the `&dyn Run` trait object to a concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Converts the `&dyn Run` trait object to a mutable concrete type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Subcommands that this command will run.
pub struct SubcommandRun {
    subcommands: Vec<Box<dyn Run>>,
}

impl fmt::Debug for SubcommandRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subcommand")
            //.field("subcommands", &self.subcommands)
            .finish()
    }
}

impl SubcommandRun {
    /// Creates a new `SubcommandRun`.
    pub fn new(subcommands: Vec<Box<dyn Run>>) -> Self {
        Self { subcommands }
    }

    /// Returns the subcommands that this command will run.
    pub fn subcommands(&self) -> &[Box<dyn Run>] {
        &self.subcommands
    }

    /// Returns the subcommands that this command will run.
    pub fn subcommands_mut(&mut self) -> &mut Vec<Box<dyn Run>> {
        &mut self.subcommands
    }

    /// Runs the subcommands.
    pub fn run(&self, config: &config::Config) -> Result<()> {
        for subcommand in &self.subcommands {
            subcommand.run(config)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct S(i32);
    impl Run for S {
        fn run(&self, _config: &config::Config) -> Result<()> {
            Ok(())
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn run_downcast() {
        let s = S(42);
        let s = &s as &(dyn Run);
        let s = s.as_any().downcast_ref::<S>().unwrap();
        assert_eq!(s.0, 42);

        let mut s = S(42);
        let s = &mut s as &mut (dyn Run);
        let s = s.as_any_mut().downcast_mut::<S>().unwrap();
        assert_eq!(s.0, 42);

        let s = S(42);
        let s = Box::new(s) as Box<dyn Run>;
        let s = s.into_any().downcast::<S>().unwrap();
        assert_eq!(s.0, 42);
    }
}
