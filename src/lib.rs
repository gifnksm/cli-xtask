#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! A collection of utility functions and command line interfaces for
//! [cargo-xtask].
//!
//! This crate provides the following utilities:
//!
//! * **[`cargo xtask dist`](crate::subcommand::Dist)** and related subcommands
//!   * Builds a distributable tar.gz package for your bin crate.
//! * **[`cargo xtask lint`](crate::subcommand::Lint)** and related subcommands
//!   * Runs the lints for your bin/lib crate.
//!   * Integrated with [`rustdoc`], [`rustfmt`], [`clippy`], [`cargo-rdme`],
//!     [`cargo-udeps`].
//! * **[`cargo xtask tidy`](crate::subcommand::Tidy)** and related subcommands
//!   * Fixes the problems on your bin/lib crate.
//!   * Integrated with  [`rustfmt`], [`clippy`], [`cargo-rdme`].
//! * **[`cargo xtask pre-release`](crate::subcommand::PreRelease)**
//!   * Checks if your bin/lib crate is ready for a release.
//! * **[`cargo xtask build`](crate::subcommand::Build),
//!   [`clippy`](crate::subcommand::Clippy), [`doc`](crate::subcommand::Doc),
//!   [`fmt`](crate::subcommand::Fmt), [`test`](crate::subcommand::Test)**
//!   * Runs the cargo commands with options useful for testing and continuous
//!     integration.
//!     * **`--all-workspaces`** - Runs the cargo commands for all workspaces.
//!     * **`--workspace`** - Runs the cargo commands for all packages in the
//!       workspace.
//!     * **`--each-features`** - Repeats to runs the cargo commands for each
//!       feature enabled.
//!     * **`--exhaustive`** - Same as `--all-workspaces --workspace
//!       --each-features`.
//! * **[`cargo xtask docsrs`](crate::subcommand::Docsrs)**
//!   * - Builds the documentation for your lib crate with configuration for
//!     [docs.rs].
//! * **[`cargo xtask exec`](crate::subcommand::Exec)**
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
//! * **`subcommand-doc`** - Enables [`cargo xtask
//!   doc`](crate::subcommand::Doc).
//! * **`subcommand-docsrs`** - Enables [`cargo xtask
//!   docsrs`](crate::subcommand::Docsrs).
//! * **`subcommand-exec`** - Enables [`cargo xtask
//!   exec`](crate::subcommand::Exec).
//! * **`subcommand-fmt`** - Enables [`cargo xtask
//!   fmt`](crate::subcommand::Fmt).
//! * **`subcommand-lint`** - Enables [`cargo xtask
//!   lint`](crate::subcommand::Lint).
//! * **`subcommand-pre-release`** - Enables [`cargo xtask
//!   pre-release`](crate::subcommand::PreRelease).
//! * **`subcommand-test`** - Enables [`cargo xtask
//!   test`](crate::subcommand::Test).
//! * **`subcommand-tidy`** - Enables [`cargo xtask
//!   tidy`](crate::subcommand::Tidy).
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
//! [`rustdoc`]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
//! [`rustfmt`]: https://github.com/rust-lang/rustfmt
//! [`clippy`]: https://github.com/rust-lang/rust-clippy
//! [`cargo-rdme`]: https://github.com/orium/cargo-rdme
//! [`cargo-udeps`]: https://github.com/est31/cargo-udeps
//! [docs.rs]: https://docs.rs/
//! [LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
//! [CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

#![doc(html_root_url = "https://docs.rs/cli-xtask/0.1.2")]

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
pub trait Run {
    /// Runs the command or subcommand.
    fn run(&self, config: &config::Config) -> Result<()>;
}
