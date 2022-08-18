mod dist;
mod dist_package;
mod dist_target;

pub use self::{
    dist::{DistConfig, DistConfigBuilder},
    dist_package::{DistPackageConfig, DistPackageConfigBuilder},
    dist_target::{DistTargetConfig, DistTargetConfigBuilder},
};

/// Configures and constructs [`Config`]
#[derive(Debug, Default)]
pub struct ConfigBuilder<'a> {
    dist: Option<DistConfig<'a>>,
}

impl<'a> ConfigBuilder<'a> {
    /// Creates a new `ConfigBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new [`DistConfig`] to the builder.
    pub fn dist(mut self, dist: DistConfig<'a>) -> Self {
        self.dist = Some(dist);
        self
    }

    /// Builds a [`Config`] from the current configuration.
    pub fn build(self) -> Config<'a> {
        Config { dist: self.dist }
    }
}

/// Configuration for cargo xtask
#[derive(Debug)]
pub struct Config<'a> {
    dist: Option<DistConfig<'a>>,
}

impl<'a> Config<'a> {
    /// Returns the [`DistConfig`] if one was configured.
    pub fn dist(&self) -> eyre::Result<&DistConfig<'a>> {
        self.dist
            .as_ref()
            .ok_or_else(|| eyre::eyre!("no dist configured"))
    }
}
