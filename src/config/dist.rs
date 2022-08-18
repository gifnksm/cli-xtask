use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, Package,
};

use super::{DistPackageConfig, DistPackageConfigBuilder};

/// Configures and constructs [`DistConfig`]
#[derive(Debug)]
pub struct DistConfigBuilder<'a> {
    name: String,
    metadata: &'a Metadata,
    dist_target_directory: Utf8PathBuf,
    dist_base_working_directory: Utf8PathBuf,
    packages: Vec<DistPackageConfig<'a>>,
}

impl<'a> DistConfigBuilder<'a> {
    /// Creates a new `DistConfigBuilder` from the given name.
    ///
    /// Created `DistConfig` will be associated with current cargo workspace.
    pub fn new(name: impl Into<String>, metadata: &'a Metadata) -> Self {
        let name = name.into();
        let dist_target_directory = metadata.target_directory.join("dist");
        let dist_base_working_directory = metadata.target_directory.join("xtask/dist").join(&name);

        Self {
            name,
            metadata,
            dist_target_directory,
            dist_base_working_directory,
            packages: vec![],
        }
    }

    /// Creates a new `DistConfigBuilder` from the root package of given workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the root package is not found.
    pub fn from_root_package(
        metadata: &'a Metadata,
    ) -> eyre::Result<(Self, DistPackageConfigBuilder<'a>)> {
        let package = metadata
            .root_package()
            .ok_or_else(|| eyre::eyre!("no root package found"))?;
        Ok(Self::from_package(metadata, package))
    }

    /// Creates a new `DistConfigBuilder` from a package with the given name in the the given workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the package with the specified name is not found.
    pub fn from_package_name(
        metadata: &'a Metadata,
        name: &str,
    ) -> eyre::Result<(Self, DistPackageConfigBuilder<'a>)> {
        let workspace_packages = metadata.workspace_packages();
        let package = workspace_packages
            .iter()
            .find(|package| package.name == name)
            .ok_or_else(|| eyre::eyre!("no package found"))?;
        Ok(Self::from_package(metadata, package))
    }

    fn from_package(
        metadata: &'a Metadata,
        package: &'a Package,
    ) -> (Self, DistPackageConfigBuilder<'a>) {
        let name = format!("{}-v{}", package.name, package.version);

        let dist = Self::new(name, metadata);
        let package_builder = DistPackageConfigBuilder::new(package);

        (dist, package_builder)
    }

    /// Adds the given package to the `DistConfig`.
    pub fn package(mut self, package: DistPackageConfig<'a>) -> Self {
        self.packages.push(package);
        self
    }

    /// Adds the given packages to the `DistConfig`.
    pub fn packages(mut self, packages: impl IntoIterator<Item = DistPackageConfig<'a>>) -> Self {
        self.packages.extend(packages);
        self
    }

    /// Builds a [`DistConfig`] from the current configuration.
    pub fn build(self) -> DistConfig<'a> {
        DistConfig {
            name: self.name,
            metadata: self.metadata,
            dist_target_directory: self.dist_target_directory,
            dist_base_working_directory: self.dist_base_working_directory,
            packages: self.packages,
        }
    }
}

/// Configuration for the distribution.
#[derive(Debug)]
pub struct DistConfig<'a> {
    name: String,
    metadata: &'a Metadata,
    dist_target_directory: Utf8PathBuf,
    dist_base_working_directory: Utf8PathBuf,
    packages: Vec<DistPackageConfig<'a>>,
}

impl<'a> DistConfig<'a> {
    /// Returns the name of the distribution.
    ///
    /// By default, the name is formed as `<package-name>-v<package-version>`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the cargo workspace [`Metadata`](cargo_metadata::Metadata).
    pub fn metadata(&self) -> &'a Metadata {
        self.metadata
    }

    /// Returns the target directory that will be used to store the distribution archive.
    pub fn dist_target_directory(&self) -> &Utf8Path {
        &self.dist_target_directory
    }

    /// Returns the base working directory where the distribution artifacts will be copied at.
    pub fn dist_base_working_directory(&self) -> &Utf8Path {
        &self.dist_base_working_directory
    }

    /// Returns the working directory where the distribution artifacts will be copied at.
    pub fn dist_working_directory(&self, target_triple: Option<&str>) -> Utf8PathBuf {
        let target_triple = target_triple.unwrap_or("noarch");
        self.dist_base_working_directory.join(target_triple)
    }

    /// Returns the configurations of the packages that will be distributed.
    pub fn packages(&self) -> &[DistPackageConfig] {
        &self.packages
    }
}
