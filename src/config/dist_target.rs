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
    command: Option<clap::Command>,
    #[cfg(feature = "subcommand-dist-build-bin")]
    cargo_build_options: Vec<String>,
}

impl<'a> DistTargetConfigBuilder<'a> {
    pub(crate) fn from_metadata(target: &'a Target) -> Self {
        Self {
            name: target.name.clone(),
            metadata: target,
            command: None,
            #[cfg(feature = "subcommand-dist-build-bin")]
            cargo_build_options: vec![],
        }
    }

    pub(crate) fn target_by_name(package: &'a Package, name: &str, kind: &str) -> Result<Self> {
        let target = package
            .targets
            .iter()
            .find(|t| t.name == name && t.kind.iter().any(|k| k == &kind.into()))
            .ok_or_else(|| eyre!("command not found: {name}, {kind}"))?;
        Ok(Self {
            name: name.to_string(),
            metadata: target,
            command: None,
            #[cfg(feature = "subcommand-dist-build-bin")]
            cargo_build_options: vec![],
        })
    }

    /// Set the command line interface definition for the target.
    pub fn command(mut self, command: clap::Command) -> Self {
        self.command = Some(command);
        self
    }

    /// Adds cargo build options to be used when building the target.
    #[cfg(feature = "subcommand-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
    pub fn cargo_build_options(
        mut self,
        options: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.cargo_build_options
            .extend(options.into_iter().map(Into::into));
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
            #[cfg(feature = "subcommand-dist-build-bin")]
            cargo_build_options: self.cargo_build_options,
        })
    }
}

/// Configuration for the distribution of the target.
#[derive(Debug)]
pub struct DistTargetConfig<'a> {
    name: String,
    metadata: &'a Target,
    command: Option<clap::Command>,
    #[cfg(feature = "subcommand-dist-build-bin")]
    cargo_build_options: Vec<String>,
}

impl DistTargetConfig<'_> {
    /// Return the name of the target.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata of the target.
    pub fn metadata(&self) -> &Target {
        self.metadata
    }

    /// Returns the command line interface definition for the target.
    pub fn command(&self) -> Option<&clap::Command> {
        self.command.as_ref()
    }

    /// Returns the cargo build options to be used when building the target.
    #[cfg(feature = "subcommand-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
    pub fn cargo_build_options(&self) -> &[String] {
        &self.cargo_build_options
    }
}
