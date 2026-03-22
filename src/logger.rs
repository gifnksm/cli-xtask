//! Utility functions for logging.

use std::{env, io};

use eyre::eyre;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::Result;

/// Install a `tracing-subscriber` as a logger.
pub fn install(verbosity: Option<Level>) -> Result<()> {
    let env_filter = if env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        let level = match verbosity {
            Some(Level::ERROR) => "error",
            Some(Level::WARN) => "warn",
            Some(Level::INFO) => "info",
            Some(Level::DEBUG) => "debug",
            Some(Level::TRACE) => "trace",
            None => "off",
        };
        EnvFilter::new(level)
    };

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(io::stderr)
        .with_target(false)
        .try_init()
        .map_err(|e| eyre!(e))?;

    Ok(())
}
