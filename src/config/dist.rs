use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, Package,
};
use eyre::eyre;

use super::{DistPackageConfig, DistPackageConfigBuilder};
use crate::Result;

/// Configures and constructs [`DistConfig`].
///
/// # Examples
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{config::DistConfigBuilder, workspace};
///
/// let workspace = workspace::current();
/// let config = DistConfigBuilder::new("app", workspace).build()?;
/// # Ok(())
/// # }
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::DistConfigBuilder, workspace};
    ///
    /// let workspace = workspace::current();
    /// let config = DistConfigBuilder::new("app", workspace).build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(name: impl Into<String>, workspace: &'a Metadata) -> Self {
        let name = name.into();
        let dist_target_directory = workspace.target_directory.join("dist");
        let dist_base_working_directory = workspace.target_directory.join("xtask/dist").join(&name);

        Self {
            name,
            metadata: workspace,
            dist_target_directory,
            dist_base_working_directory,
            packages: vec![],
        }
    }

    /// Creates a new `DistConfigBuilder` from the root package of given
    /// workspace.
    ///
    /// The name of the created `DistConfig` will be generated from the name and
    /// version of the root package.
    ///
    /// # Errors
    ///
    /// Returns an error if the root package is not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::DistConfigBuilder, workspace};
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_root_package(workspace)?;
    /// let dist_config = dist_config.package(pkg_config.build()?).build()?;
    ///
    /// let root_package = workspace.root_package().unwrap();
    /// assert_eq!(
    ///     dist_config.name(),
    ///     format!("{}-v{}", root_package.name, root_package.version)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_root_package(
        workspace: &'a Metadata,
    ) -> Result<(Self, DistPackageConfigBuilder<'a>)> {
        let package = workspace
            .root_package()
            .ok_or_else(|| eyre!("no root package found"))?;
        Ok(Self::from_package(workspace, package))
    }

    /// Creates a new `DistConfigBuilder` from a package with the given name in
    /// the the given workspace.
    ///
    /// The name of the created `DistConfig` will be generated from the name and
    /// version of the given package.
    ///
    /// # Errors
    ///
    /// Returns an error if the package with the specified name is not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::DistConfigBuilder, workspace};
    ///
    /// let workspace = workspace::current();
    /// let package = workspace.workspace_packages()[0];
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, &package.name)?;
    /// let dist_config = dist_config.package(pkg_config.build()?).build()?;
    ///
    /// assert_eq!(
    ///     dist_config.name(),
    ///     format!("{}-v{}", package.name, package.version)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_package_name(
        workspace: &'a Metadata,
        name: &str,
    ) -> Result<(Self, DistPackageConfigBuilder<'a>)> {
        let workspace_packages = workspace.workspace_packages();
        let package = workspace_packages
            .iter()
            .find(|package| package.name == name)
            .ok_or_else(|| eyre!("no package found"))?;
        Ok(Self::from_package(workspace, package))
    }

    fn from_package(
        workspace: &'a Metadata,
        package: &'a Package,
    ) -> (Self, DistPackageConfigBuilder<'a>) {
        let name = format!("{}-v{}", package.name, package.version);

        let dist = Self::new(name, workspace);
        let package_builder = DistPackageConfigBuilder::new(package);

        (dist, package_builder)
    }

    /// Creates a new [`DistPackageConfigBuilder`] from the given package name.
    ///
    /// # Errors
    ///
    /// Returns an error if the package with the specified name is not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::DistConfigBuilder, workspace};
    ///
    /// let workspace = workspace::current();
    /// let package = workspace.workspace_packages()[0];
    ///
    /// let dist_config = DistConfigBuilder::new("app-dist", workspace);
    /// let pkg_config = dist_config.package_by_name(&package.name)?.build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn package_by_name(&self, name: &str) -> Result<DistPackageConfigBuilder<'a>> {
        let package = self
            .metadata
            .workspace_packages()
            .into_iter()
            .find(|package| package.name == name)
            .ok_or_else(|| eyre!("no package found"))?;
        let package_builder = DistPackageConfigBuilder::new(package);
        Ok(package_builder)
    }

    /// Adds the given package to the `DistConfig`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::DistConfigBuilder, workspace};
    ///
    /// let workspace = workspace::current();
    /// let package = workspace.workspace_packages()[0];
    ///
    /// let dist_config = DistConfigBuilder::new("app-dist", workspace);
    /// let pkg_config = dist_config.package_by_name(&package.name)?.build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    /// # Ok(())
    /// # }
    pub fn package(mut self, package: DistPackageConfig<'a>) -> Self {
        self.packages.push(package);
        self
    }

    /// Adds the given packages to the `DistConfig`.
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{config::{DistConfigBuilder, DistPackageConfig}, workspace, Result};
    ///
    /// let workspace = workspace::current();
    /// let packages = workspace.workspace_packages();
    ///
    /// let dist_config = DistConfigBuilder::new("app-dist", workspace);
    /// let pkg_configs = packages.iter().map(|package| -> Result<DistPackageConfig> {
    ///     let pkg_config = dist_config.package_by_name(&package.name)?.build()?;
    ///     Ok(pkg_config)
    /// }).collect::<Result<Vec<_>>>()?;
    /// let dist_config = dist_config.packages(pkg_configs).build()?;
    /// # Ok(())
    /// # }
    pub fn packages(mut self, packages: impl IntoIterator<Item = DistPackageConfig<'a>>) -> Self {
        self.packages.extend(packages);
        self
    }

    /// Builds a [`DistConfig`] from the current configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistConfig`] cannot be built.
    pub fn build(self) -> Result<DistConfig<'a>> {
        Ok(DistConfig {
            name: self.name,
            metadata: self.metadata,
            dist_target_directory: self.dist_target_directory,
            dist_base_working_directory: self.dist_base_working_directory,
            packages: self.packages,
        })
    }
}

/// Configuration for the distribution.
///
/// This struct is build from [`DistConfigBuilder`].
///
/// # Examples
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{config::DistConfigBuilder, workspace};
///
/// let workspace = workspace::current();
/// let config = DistConfigBuilder::new("app", workspace).build()?;
/// # Ok(())
/// # }
/// ```
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

    /// Returns the target directory that will be used to store the distribution
    /// archive.
    pub fn dist_target_directory(&self) -> &Utf8Path {
        &self.dist_target_directory
    }

    /// Returns the base working directory where the distribution artifacts will
    /// be copied at.
    pub fn dist_base_working_directory(&self) -> &Utf8Path {
        &self.dist_base_working_directory
    }

    /// Returns the working directory where the distribution artifacts will be
    /// copied at.
    pub fn dist_working_directory(&self, target_triple: Option<&str>) -> Utf8PathBuf {
        let target_triple = target_triple.unwrap_or("noarch");
        self.dist_base_working_directory.join(target_triple)
    }

    /// Returns the configurations of the packages that will be distributed.
    pub fn packages(&self) -> &[DistPackageConfig] {
        &self.packages
    }
}
