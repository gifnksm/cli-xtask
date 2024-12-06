#[cfg(any(
    feature = "subcommand-dist-build-license",
    feature = "subcommand-dist-build-doc"
))]
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::{camino::Utf8Path, Package};

use super::{DistTargetConfig, DistTargetConfigBuilder};
use crate::{workspace::PackageExt, Result};

/// Configures and constructs [`DistPackageConfig`].
///
/// This struct is build from [`DistConfigBuilder`](super::DistConfigBuilder).
///
/// # Examples
///
/// Creates [`DistConfigBuilder`](super::DistConfigBuilder) and
/// `DistPackageConfigBuilder` from the workspace root package:
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{config::DistConfigBuilder, workspace};
///
/// let workspace = workspace::current();
///
/// let (dist_config, pkg_config) = DistConfigBuilder::from_root_package(workspace)?;
/// # Ok(())
/// # }
/// ```
///
/// Creates [`DistConfigBuilder`](super::DistConfigBuilder) and
/// `DistPackageConfigBuilder` from the name of package:
///
/// ```rust
/// # fn main() -> cli_xtask::Result<()> {
/// use cli_xtask::{config::DistConfigBuilder, workspace};
///
/// let workspace = workspace::current();
/// let package = workspace.workspace_packages()[0];
///
/// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, &package.name)?;
/// # Ok(())
/// # }
/// ```
///
/// Creates `DistPackageConfigBuilder` from the name of package and
/// [`DistConfigBuilder`](super::DistConfigBuilder):
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
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct DistPackageConfigBuilder<'a> {
    name: String,
    metadata: &'a Package,
    targets: Option<Vec<DistTargetConfig<'a>>>,
    #[cfg(feature = "subcommand-dist-build-bin")]
    cargo_build_options: Vec<String>,
    #[cfg(feature = "subcommand-dist-build-license")]
    license_files: Option<Vec<Utf8PathBuf>>,
    #[cfg(feature = "subcommand-dist-build-doc")]
    documents: Option<Vec<Utf8PathBuf>>,
}

impl<'a> DistPackageConfigBuilder<'a> {
    pub(crate) fn new(package: &'a Package) -> Self {
        Self {
            name: package.name.clone(),
            metadata: package,
            targets: None,
            #[cfg(feature = "subcommand-dist-build-bin")]
            cargo_build_options: vec![],
            #[cfg(feature = "subcommand-dist-build-license")]
            license_files: None,
            #[cfg(feature = "subcommand-dist-build-doc")]
            documents: None,
        }
    }

    /// Creates new `DistTargetConfigBuilder`s from all binary targets of the
    /// package.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistTargetConfig`] cannot be built.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "xtask")?;
    /// let target_builders = pkg_config.all_binaries();
    /// let targets = target_builders
    ///     .into_iter()
    ///     .map(DistTargetConfigBuilder::build)
    ///     .collect::<Result<Vec<_>>>()?;
    /// let pkg_config = pkg_config.targets(targets).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn all_binaries(&self) -> Vec<DistTargetConfigBuilder<'a>> {
        collect_targets(self.metadata, "bin")
    }

    /// Creates new `DistTargetConfigBuilder`s from given kind of targets in the
    /// package.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistTargetConfig`] cannot be built.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "cli-xtask")?;
    /// let target_builders = pkg_config.all_targets("bin");
    /// let targets = target_builders
    ///     .into_iter()
    ///     .map(DistTargetConfigBuilder::build)
    ///     .collect::<Result<Vec<_>>>()?;
    /// let pkg_config = pkg_config.targets(targets).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "cli-xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn all_targets(&self, kind: &str) -> Vec<DistTargetConfigBuilder<'a>> {
        collect_targets(self.metadata, kind)
    }

    /// Create a new `DistTargetConfigBuilder` from the name of the binary
    /// target.
    ///
    /// # Errors
    ///
    /// Returns an error if the binary target with the given name is not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     clap::CommandFactory,
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "xtask")?;
    /// let target_builder = pkg_config.binary_by_name("xtask")?;
    /// let target = target_builder.build()?;
    /// let pkg_config = pkg_config.target(target).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn binary_by_name(&self, name: &str) -> Result<DistTargetConfigBuilder<'a>> {
        DistTargetConfigBuilder::target_by_name(self.metadata, name, "bin")
    }

    /// Create a new `DistTargetConfigBuilder` from the name and kind of the
    /// target.
    ///
    /// # Errors
    ///
    /// Returns an error if the target with the given name and kind is not
    /// found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     clap::CommandFactory,
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "cli-xtask")?;
    /// let target_builder = pkg_config.target_by_name("cli-xtask", "bin")?;
    /// let target = target_builder.build()?;
    /// let pkg_config = pkg_config.target(target).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "cli-xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn target_by_name(&self, name: &str, kind: &str) -> Result<DistTargetConfigBuilder<'a>> {
        DistTargetConfigBuilder::target_by_name(self.metadata, name, kind)
    }

    /// Add a target of the package to the list of targets to be distributed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     clap::CommandFactory,
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "cli-xtask")?;
    /// let target_builder = pkg_config.target_by_name("cli-xtask", "bin")?;
    /// let target = target_builder.build()?;
    /// let pkg_config = pkg_config.target(target).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "cli-xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn target(mut self, target: DistTargetConfig<'a>) -> Self {
        self.targets.get_or_insert(vec![]).push(target);
        self
    }

    /// Add a targets of the package to the list of targets to be distributed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> cli_xtask::Result<()> {
    /// use cli_xtask::{
    ///     config::{DistConfigBuilder, DistTargetConfigBuilder},
    ///     workspace, Result,
    /// };
    ///
    /// let workspace = workspace::current();
    ///
    /// let (dist_config, pkg_config) = DistConfigBuilder::from_package_name(workspace, "cli-xtask")?;
    /// let target_builders = pkg_config.all_targets("bin");
    /// let targets = target_builders
    ///     .into_iter()
    ///     .map(DistTargetConfigBuilder::build)
    ///     .collect::<Result<Vec<_>>>()?;
    /// let pkg_config = pkg_config.targets(targets).build()?;
    /// let dist_config = dist_config.package(pkg_config).build()?;
    ///
    /// let target = &dist_config.packages()[0].targets()[0];
    /// assert_eq!(target.name(), "cli-xtask");
    /// # Ok(())
    /// # }
    /// ```
    pub fn targets(mut self, targets: impl IntoIterator<Item = DistTargetConfig<'a>>) -> Self {
        self.targets.get_or_insert(vec![]).extend(targets);
        self
    }

    /// Adds cargo build options to be used when building the package.
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
    /// let pkg_config = pkg_config.cargo_build_options(["--features", "feature-a"]);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Adds a package license files to the list of files to be distributed.
    ///
    /// If the given path is a relative path, it is resolved against the package
    /// root direcotry.
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
    /// let pkg_config = pkg_config.license_files(
    ///     ["LICENSE-MIT", "LICENSE-APACHE"]
    ///         .into_iter()
    ///         .map(Into::into),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "subcommand-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
    pub fn license_files(mut self, files: impl IntoIterator<Item = Utf8PathBuf>) -> Self {
        let package_root = self.metadata.root_directory();
        let files = files.into_iter().map(|file| {
            if file.is_relative() {
                package_root.join(file)
            } else {
                file
            }
        });
        match &mut self.license_files {
            Some(fs) => fs.extend(files),
            lf @ None => *lf = Some(files.collect()),
        }
        self
    }

    /// Adds a package documentation files to the list of files to be
    /// distributed.
    ///
    /// If the given path is a relative path, it is resolved against the package
    /// root direcotry.
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
    /// let pkg_config = pkg_config.documents(["CHANGELOG.md"].into_iter().map(Into::into));
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "subcommand-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
    pub fn documents(mut self, files: impl IntoIterator<Item = Utf8PathBuf>) -> Self {
        let package_root = self.metadata.root_directory();
        let files = files.into_iter().map(|file| {
            if file.is_relative() {
                package_root.join(file)
            } else {
                file
            }
        });
        match &mut self.documents {
            Some(ds) => ds.extend(files),
            ds @ None => *ds = Some(files.collect()),
        }
        self
    }

    /// Builds a [`DistPackageConfig`] from the current configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`DistPackageConfig`] cannot be built.
    pub fn build(self) -> Result<DistPackageConfig<'a>> {
        let targets = match self.targets {
            Some(targets) => targets,
            None => collect_targets(self.metadata, "bin")
                .into_iter()
                .map(DistTargetConfigBuilder::build)
                .collect::<Result<Vec<_>>>()?,
        };
        Ok(DistPackageConfig {
            name: self.name,
            metadata: self.metadata,
            targets,
            #[cfg(feature = "subcommand-dist-build-bin")]
            cargo_build_options: self.cargo_build_options,
            #[cfg(feature = "subcommand-dist-build-license")]
            license_files: collect_license_files(self.metadata, self.license_files)?,
            #[cfg(feature = "subcommand-dist-build-doc")]
            documents: self.documents.unwrap_or_default(),
        })
    }
}

/// Configuration for the distribution of the package.
#[derive(Debug)]
pub struct DistPackageConfig<'a> {
    name: String,
    metadata: &'a Package,
    targets: Vec<DistTargetConfig<'a>>,
    #[cfg(feature = "subcommand-dist-build-bin")]
    cargo_build_options: Vec<String>,
    #[cfg(feature = "subcommand-dist-build-license")]
    license_files: Vec<Utf8PathBuf>,
    #[cfg(feature = "subcommand-dist-build-doc")]
    documents: Vec<Utf8PathBuf>,
}

impl<'a> DistPackageConfig<'a> {
    /// Returns the name of the package.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata of the package.
    pub fn metadata(&self) -> &'a Package {
        self.metadata
    }

    /// Returns the list of targets to be distributed.
    ///
    /// Targets can be added by associated functions of
    /// [`DistPackageConfigBuilder`].
    ///
    /// If no targets are added, the list of targets is constructed from all
    /// binaries target of the package.
    pub fn targets(&self) -> &[DistTargetConfig] {
        &self.targets
    }

    /// Returns the path to the package's root directory.
    pub fn root_directory(&self) -> &Utf8Path {
        self.metadata.root_directory()
    }

    /// Returns the list of cargo build options to be used when building the
    #[cfg(feature = "subcommand-dist-build-bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-bin")))]
    pub fn cargo_build_options(&self) -> &[String] {
        &self.cargo_build_options
    }

    /// Returns the list of license files to be distributed.
    ///
    /// License files can be added by
    /// [`DistPackageConfigBuilder::license_files`] function.
    ///
    /// If no license files are added, the list of license files is constructed
    /// from the `license-file` field of the `[package]` section of the
    /// manifest.
    ///
    /// If no license files are added and the `license-file` field is not
    /// present, the file matches the pattern `/^LICENSE(?:-|_|\.|$)/i` in the
    /// root directory of the package.
    #[cfg(feature = "subcommand-dist-build-license")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-license")))]
    pub fn license_files(&self) -> &[Utf8PathBuf] {
        &self.license_files
    }

    /// Returns the list of documentation files to be distributed.
    ///
    /// Documentation files can be added by
    /// [`DistPackageConfigBuilder::documents`] function.
    /// If no documentation files are added, this function returns empty list.
    #[cfg(feature = "subcommand-dist-build-doc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "subcommand-dist-build-doc")))]
    pub fn documents(&self) -> &[Utf8PathBuf] {
        &self.documents
    }
}

fn collect_targets<'a>(package: &'a Package, kind: &str) -> Vec<DistTargetConfigBuilder<'a>> {
    package
        .targets
        .iter()
        .filter(|target| target.kind.iter().any(|x| x == &kind.into()))
        .map(DistTargetConfigBuilder::from_metadata)
        .collect()
}

#[cfg(feature = "subcommand-dist-build-license")]
fn collect_license_files(
    package: &Package,
    files: Option<Vec<Utf8PathBuf>>,
) -> Result<Vec<Utf8PathBuf>> {
    use once_cell::sync::Lazy;
    use regex::{Regex, RegexBuilder};
    let src_dir = package.root_directory();

    if let Some(files) = files {
        return Ok(files);
    }

    if let Some(license_file) = &package.license_file {
        return Ok(vec![src_dir.join(license_file)]);
    }

    let mut files = vec![];
    for src_entry in src_dir.read_dir_utf8()? {
        let src_entry = src_entry?;
        if !src_entry.file_type()?.is_file() {
            continue;
        }

        let src_file = src_entry.path();
        static RE: Lazy<Regex> = Lazy::new(|| {
            RegexBuilder::new(r"^LICENSE(?:-|_|\.|$)")
                .case_insensitive(true)
                .build()
                .unwrap()
        });

        let src_name = match src_file.file_name() {
            Some(name) => name,
            None => continue,
        };
        if !RE.is_match(src_name) {
            continue;
        }
        files.push(src_file.to_owned());
    }

    Ok(files)
}
