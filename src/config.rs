//! Data structures for workflow configuration.

mod dist;
mod dist_package;
mod dist_target;

use eyre::eyre;

pub use self::{
    dist::{DistConfig, DistConfigBuilder},
    dist_package::{DistPackageConfig, DistPackageConfigBuilder},
    dist_target::{DistTargetConfig, DistTargetConfigBuilder},
};
use crate::Result;

/// Configures and constructs [`Config`].
///
/// # Examples
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{
///     config::{ConfigBuilder, DistConfigBuilder},
///     workspace,
/// };
///
/// let workspace = workspace::current();
/// let dist_config = DistConfigBuilder::new("app-dist", workspace).build()?;
/// let config = ConfigBuilder::new().dist(dist_config).build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct ConfigBuilder<'a> {
    dist: Option<DistConfig<'a>>,
}

impl<'a> ConfigBuilder<'a> {
    /// Creates a new `ConfigBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a [`DistConfig`] to the builder.
    pub fn dist(mut self, dist: DistConfig<'a>) -> Self {
        self.dist = Some(dist);
        self
    }

    /// Builds a [`Config`] from the current configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Config`] cannot be built.
    pub fn build(self) -> Result<Config<'a>> {
        Ok(Config { dist: self.dist })
    }
}

/// Top-level configuration for cargo xtask workflow.
///
/// This struct is build from [`ConfigBuilder`].
///
/// # Examples
///
/// Creates an empty `Config`.
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::config::Config;
/// let config = Config::new();
/// assert!(config.dist().is_err());
/// # Ok(())
/// # }
/// ```
///
/// Creates a `Config` with a [`DistConfig`].
///
/// ```
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{
///     config::{ConfigBuilder, DistConfigBuilder},
///     workspace,
/// };
///
/// let workspace = workspace::current();
/// let dist_config = DistConfigBuilder::new("app-dist", workspace).build()?;
/// let config = ConfigBuilder::new().dist(dist_config).build()?;
/// assert!(config.dist().is_ok());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Config<'a> {
    dist: Option<DistConfig<'a>>,
}

impl<'a> Config<'a> {
    /// Creates an empty `Config`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the [`DistConfig`] if one was configured.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistConfig`] is not set.
    pub fn dist(&self) -> Result<&DistConfig<'a>> {
        self.dist
            .as_ref()
            .ok_or_else(|| eyre!("no dist configuration set"))
    }
}
