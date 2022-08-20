use cargo_metadata::{Package, Target};
use eyre::eyre;

use crate::Result;

/// Configures and constructs [`DistTargetConfig`].
///
/// This struct is build from
/// [`DistPackageConfigBuilder`](super::DistPackageConfigBuilder).
#[derive(Debug)]
pub struct DistTargetConfigBuilder<'a> {
    name: String,
    metadata: &'a Target,
    command: Option<clap::Command<'static>>,
}

impl<'a> DistTargetConfigBuilder<'a> {
    pub(crate) fn from_metadata(target: &'a Target) -> Self {
        Self {
            name: target.name.clone(),
            metadata: target,
            command: None,
        }
    }

    pub(crate) fn target_by_name(package: &'a Package, name: &str, kind: &str) -> Result<Self> {
        let target = package
            .targets
            .iter()
            .find(|t| t.name == name && t.kind.iter().any(|k| k == kind))
            .ok_or_else(|| eyre!("command not found: {name}, {kind}"))?;
        Ok(Self {
            name: name.to_string(),
            metadata: target,
            command: None,
        })
    }

    /// Set the command line interface definition for the target.
    pub fn command(mut self, command: clap::Command<'static>) -> Self {
        self.command = Some(command);
        self
    }

    /// Builds a [`DistTargetConfig`] from the current configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistTargetConfig`] cannot be built.
    pub fn build(self) -> Result<DistTargetConfig<'a>> {
        Ok(DistTargetConfig {
            name: self.name,
            metadata: self.metadata,
            command: self.command,
        })
    }
}

/// Configuration for the distribution of the target.
#[derive(Debug)]
pub struct DistTargetConfig<'a> {
    name: String,
    metadata: &'a Target,
    command: Option<clap::Command<'static>>,
}

impl<'a> DistTargetConfig<'a> {
    /// Return the name of the target.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata of the target.
    pub fn metadata(&self) -> &Target {
        self.metadata
    }

    /// Returns the command line interface definition for the target.
    pub fn command(&self) -> Option<&clap::Command<'static>> {
        self.command.as_ref()
    }
}
