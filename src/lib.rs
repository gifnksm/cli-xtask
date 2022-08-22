#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A collection of utility functions and command line interfaces for
//! [cargo-xtask].
//!
//! This crate provides the following utilities:
//!
//! * **`cargo xtask dist`** and related subcommands - Builds a distributable
//!   tar.gz package for your bin crate.
//! * **`cargo xtask lint`** and related subcommands - Runs the lints for your
//!   bin/lib crate.
//!   * Integrated with  [`rustfmt`], [`clippy`], [`cargo-rdme`],
//!     [`cargo-udeps`].
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
//!     If you want to use extra tools such as `cargo-rdme` and `cargo-udeps`,
//!     add the `bin-crate-extra` feature.
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
//!     If you want to use extra tools such as `cargo-rdme` and `cargo-udeps`,
//!     add the `lib-crate-extra` feature.
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
//! ```console
//! $ cargo xtask help
//! cargo-xtask
//! Rust project automation command
//!
//! USAGE:
//!     cargo xtask [OPTIONS] [SUBCOMMAND]
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -q, --quiet      Less output per occurrence
//!     -v, --verbose    More output per occurrence
//!
//! SUBCOMMANDS:
//!     build                    Run `cargo build` on all workspaces in the current directory and
//!                                  subdirectories
//!     clippy                   Run `cargo clippy` on all workspaces in the current directory and
//!                                  subdirectories
//!     dist                     Create the archive file for distribution
//!     dist-archive             Create the archive file for distribution
//!     dist-build               Build all artifacts for distribution
//!     dist-build-bin           Build the release binaries dor distribution
//!     dist-build-completion    Build the shell completion files for distribution
//!     dist-build-doc           Build the documentation for distribution
//!     dist-build-license       Build the license files for distribution
//!     dist-build-man           Build the man pages for distribution
//!     dist-build-readme        Build the readme files for distribution
//!     dist-clean               Removes the dist artifacts
//!     fmt                      Run `cargo fmt` on all workspaces in the current directory and
//!                                  subdirectories
//!     help                     Print this message or the help of the given subcommand(s)
//!     lint                     Run all lint commands on all workspaces in the current directory
//!                                  and subdirectories
//!     rdme                     Run `cargo rdme` on all workspaces in the current directory and
//!                                  subdirectories
//!     test                     Run `cargo test` on all workspaces in the current directory and
//!                                  subdirectories
//!     udeps                    Run `cargo udeps` on all workspaces in the current directory and
//!                                  subdirectories
//! ```
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
//! [`clap::Subcommand`](crate::clap::Subcommand) and [`Run`](crate::Run).
//! See [the documentation of `Xtask`](crate::Xtask) for more
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
//! * **`main`** - Enables [`main`](crate::Xtask::main) function and
//!   [`main_with_config`](crate::Xtask::main_with_config) function that are the
//!   premade entry point for the CLI.
//! * **`error-handler`** - Enables functions for error handling in
//!   [`error_handler`](crate::error_handler) module.
//! * **`logger`** - Enables functions for logging in [`logger`](crate::logger)
//!   module.
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
//! * **`subcommand-build`** - Enables [`cargo xtask
//!   build`](crate::subcommand::Build).
//! * **`subcommand-clippy`** - Enables [`cargo xtask
//!   clippy`](crate::subcommand::Clippy).
//! * **`subcommand-dist`** - Enables [`cargo xtask
//!   dist`](crate::subcommand::Dist).
//! * **`subcommand-dist-archive`** - Enables [`cargo xtask
//!   dist-archive`](crate::subcommand::DistArchive).
//! * **`subcommand-dist-build-bin`** - Enables [`cargo xtask
//!   dist-build-bin`](crate::subcommand::DistBuildBin).
//! * **`subcommand-dist-build-completion`** - Enables [`cargo xtask
//!   dist-build-completion`](crate::subcommand::DistBuildCompletion).
//! * **`subcommand-dist-build-doc`** - Enables [`cargo xtask
//!   dist-build-doc`](crate::subcommand::DistBuildDoc).
//! * **`subcommand-dist-build-license`** - Enables [`cargo xtask
//!   dist-build-license`](crate::subcommand::DistBuildLicense).
//! * **`subcommand-dist-build-man`** - Enables [`cargo xtask
//!   dist-build-man`](crate::subcommand::DistBuildMan).
//! * **`subcommand-dist-build-readme`** - Enables [`cargo xtask
//!   dist-build-readme`](crate::subcommand::DistBuildReadme).
//! * **`subcommand-dist-clean`** - Enables [`cargo xtask
//!   dist-clean`](crate::subcommand::DistClean).
//! * **`subcommand-exec`** - Enables [`cargo xtask
//!   exec`](crate::subcommand::Exec).
//! * **`subcommand-fmt`** - Enables [`cargo xtask
//!   fmt`](crate::subcommand::Fmt).
//! * **`subcommand-lint`** - Enables [`cargo xtask
//!   lint`](crate::subcommand::Lint).
//! * **`subcommand-test`** - Enables [`cargo xtask
//!   test`](crate::subcommand::Test).
//!
//! The following features require third-party tools:
//!
//! * **`subcommand-rdme`** - Enables [`cargo xtask
//!   rdme`](crate::subcommand::Rdme). Requires [`cargo-rdme`] installed.
//! * **`subcommand-udeps`** - Enables [`cargo xtask
//!   udeps`](crate::subcommand::Udeps). Requires [`cargo-udeps`] installed.
//!
//! ## Other features
//!
//! * **`archive`** - Enables [`archive`](crate::archive) module which provides
//!   the functionality to create the archive file for distribution.
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The minimum supported Rust version is **Rust 1.60.0**.
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
//! [`rustfmt`]: https://github.com/rust-lang/rustfmt
//! [`clippy`]: https://github.com/rust-lang/rust-clippy
//! [`cargo-rdme`]: https://github.com/orium/cargo-rdme
//! [`cargo-udeps`]: https://github.com/est31/cargo-udeps
//! [LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
//! [CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

#![doc(html_root_url = "https://docs.rs/cli-xtask/0.0.0")]

pub use cargo_metadata::{self, camino};
pub use clap;
pub use eyre;
pub use tracing;

#[cfg(feature = "main")]
#[cfg_attr(docsrs, doc(cfg(feature = "main")))]
mod main;

#[cfg(feature = "error-handler")]
#[cfg_attr(docsrs, doc(cfg(feature = "error-handler")))]
pub use color_eyre;

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

/// Runs the command workflow.
pub trait Run {
    /// Runs the command workflow.
    fn run(&self, config: &config::Config) -> Result<()>;
}
