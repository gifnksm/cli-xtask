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

use cargo_metadata::{Metadata, MetadataCommand};
use once_cell::sync::Lazy;

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

// module definition & exports
feature_archive! {
    /// Utilities for creating archives.
    pub mod archive;
}
feature_cargo! {
    /// Utilities for working with Cargo.
    pub mod cargo;
}
feature_command! {
    /// Command line interfaces for xtask workflows.
    pub mod command;
    pub use command::Command;
}
/// Utility functions for working with paths.
pub mod fs;

mod config;
pub use config::{
    Config, ConfigBuilder, DistConfig, DistConfigBuilder, PackageConfig, PackageConfigBuilder,
    TargetConfig, TargetConfigBuilder,
};

/// Returns a cargo workspace metadata.
pub fn cargo_workspace() -> &'static Metadata {
    static METADATA: Lazy<Metadata> = Lazy::new(|| MetadataCommand::new().exec().unwrap());
    &*METADATA
}

feature_logger! {
    /// Install a tracing-subscriber as a logger.
    pub fn install_logger() -> eyre::Result<()> {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "info");
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
    /// Install a color-eyre as a error/panic handler.
    pub fn install_error_handler() -> eyre::Result<()> {
        color_eyre::install()?;
        Ok(())
    }
}

feature_main! {
    /// Entry point for xtask crate.
    pub fn main() -> eyre::Result<()> {
        install_error_handler()?;
        install_logger()?;

        tracing::info!("Running on {}", std::env::current_dir()?.display());

        #[cfg(command)]
        {
            let metadata = cargo_workspace();
            let (dist, package) = DistConfigBuilder::from_root_package(metadata)?;
            let dist = dist.package(package.all_binaries().build()).build();
            let config = ConfigBuilder::new().dist(dist).build();
            <Command as clap::Parser>::parse().run(&config)?;
        }

        Ok(())
    }
}
