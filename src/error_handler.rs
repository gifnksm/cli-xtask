//! Utility functions for error handling.

use crate::Result;

/// Install a `color-eyre` as an error/panic handler.
pub fn install() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}
