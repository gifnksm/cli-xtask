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
    DistConfig, DistConfigBuilder, PackageConfig, PackageConfigBuilder, TargetConfig,
    TargetConfigBuilder,
};

/// Get a cargo workspace metadata.
pub fn get_metadata() -> &'static Metadata {
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
        #[cfg(feature = "error-handler")]
        install_error_handler()?;

        #[cfg(feature = "logger")]
        install_logger()?;

        #[cfg(command)]
        {
            let metadata = get_metadata();
            let (dist, package) = DistConfigBuilder::from_root_package(metadata)?;
            let dist = dist.package(package.all_binaries().build()).build();
            <Command as clap::Parser>::parse().run(&dist)?;
        }

        Ok(())
    }
}
