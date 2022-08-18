#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! cargo-xtask workflow collection for CLI application.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cli-xtask = "0.0.0"
//! ```
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The minimum supported Rust version is **Rust 1.60.0**.
//! At least the last 3 versions of stable Rust are supported at any given time.
//!
//! While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
//! Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.
//!
//! # License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0
//!    ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license
//!    ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.
//!
//! See [CONTRIBUTING.md].
//!
//! [LICENSE-APACHE]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/gifnksm/cli-xtask/blob/main/LICENSE-MIT
//! [CONTRIBUTING.md]: https://github.com/gifnksm/cli-xtask/blob/main/CONTRIBUTING.md

#![doc(html_root_url = "https://docs.rs/cli-xtask/0.0.0")]

#[macro_use]
mod macros;

// re-exports
pub use {
    cargo_metadata::{self, camino},
    eyre,
};
feature_clap_command! {
    pub use clap;
}
feature_error_handler! {
    pub use color_eyre;
}

// Module definition & exports
feature_archive! {
    /// Utilities for creating archives.
    pub mod archive;
}
feature_cargo! {
    /// Utilities for Cargo command execution.
    pub mod cargo;
}
feature_command! {
    /// Command line interfaces for xtask workflows.
    pub mod command;
    pub use command::Command;
}

feature_args! {
    mod args;
    pub use crate::args::{Args, Verbosity};
}

mod config;
/// Utility functions for working with paths.
pub mod fs;
/// Utility functions for working with processes.
pub mod process;
/// Utility functions for working with workspaces.
pub mod workspace;

pub use crate::config::{
    Config, ConfigBuilder, DistConfig, DistConfigBuilder, PackageConfig, PackageConfigBuilder,
    TargetConfig, TargetConfigBuilder,
};

feature_logger! {
    /// Install a `tracing-subscriber` as a logger.
    pub fn install_logger(verbosity: Option<tracing::Level>) -> eyre::Result<()> {
        if std::env::var_os("RUST_LOG").is_none() {
            use tracing::Level;
            use std::env;
            match verbosity {
                Some(Level::ERROR) => env::set_var("RUST_LOG", "error"),
                Some(Level::WARN) => env::set_var("RUST_LOG", "warn"),
                Some(Level::INFO) => env::set_var("RUST_LOG", "info"),
                Some(Level::DEBUG) => env::set_var("RUST_LOG", "debug"),
                Some(Level::TRACE) => env::set_var("RUST_LOG", "trace"),
                None => env::set_var("RUST_LOG", "off"),
            }
        }
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_writer(std::io::stderr)
            .with_target(false)
            .try_init()
            .map_err(|e| eyre::eyre!(e))?;

        Ok(())
    }
}

feature_error_handler! {
    /// Install a `color-eyre` as a error/panic handler.
    pub fn install_error_handler() -> eyre::Result<()> {
        color_eyre::install()?;
        Ok(())
    }
}

feature_main! {
    /// Entry point for xtask crate.
    pub fn main() -> eyre::Result<()> {
        let args = <Args as clap::Parser>::parse();

        install_error_handler()?;
        install_logger(args.verbosity())?;

        tracing::info!("Running on {}", std::env::current_dir()?.display());

        let metadata = workspace::current();
        let (dist, package) = DistConfigBuilder::from_root_package(metadata)?;
        let dist = dist.package(package.all_binaries().build()).build();
        let config = ConfigBuilder::new().dist(dist).build();
        args.run(&config)?;

        Ok(())
    }
}
