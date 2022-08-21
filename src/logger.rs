//! Utility functions for logging.

use std::{env, io};

use eyre::eyre;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::Result;

/// Install a `tracing-subscriber` as a logger.
pub fn install(verbosity: Option<Level>) -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
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
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .with_target(false)
        .try_init()
        .map_err(|e| eyre!(e))?;

    Ok(())
}
