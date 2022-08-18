#[cfg(any(
    feature = "command-dist-build-license",
    feature = "command-dist-build-doc"
))]
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::{camino::Utf8Path, Package};

use super::{DistTargetConfig, DistTargetConfigBuilder};

/// Configures and constructs [`DistPackageConfig`]
#[derive(Debug)]
pub struct DistPackageConfigBuilder<'a> {
    name: String,
    package: &'a Package,
    targets: Option<Vec<DistTargetConfig<'a>>>,
    #[cfg(feature = "command-dist-build-license")]
    license_files: Option<Vec<Utf8PathBuf>>,
    #[cfg(feature = "command-dist-build-doc")]
    documents: Option<Vec<Utf8PathBuf>>,
}

impl<'a> DistPackageConfigBuilder<'a> {
    pub(crate) fn new(package: &'a Package) -> Self {
        Self {
            name: package.name.clone(),
            package,
            targets: None,
            #[cfg(feature = "command-dist-build-license")]
            license_files: None,
            #[cfg(feature = "command-dist-build-doc")]
            documents: None,
        }
    }

    /// Add all binarie targets of the package to the list of binaries to be distributed.
    pub fn all_binaries(mut self) -> Self {
        let it = self
            .package
            .targets
            .iter()
            .filter(|target| target.kind.iter().any(|x| x == "bin"))
            .map(DistTargetConfigBuilder::from_metadata)
            .map(DistTargetConfigBuilder::build);
        match &mut self.targets {
            Some(binaries) => binaries.extend(it),
            e @ None => *e = Some(it.collect()),
        }
        self
    }

    /// Add a target of the package to the list of targets to be distributed.
    pub fn target(mut self, target: DistTargetConfig<'a>) -> Self {
        self.targets.get_or_insert(vec![]).push(target);
        self
    }

    feature_clap_command! {
        /// Add a binary target to the list of targets to be distributed.
        ///
        /// # Errors
        ///
        /// Returns an error if the binary target with the given command name is not found.
        pub fn binary_from_command(mut self, command: clap::Command<'static>) -> eyre::Result<Self> {
            let binary = DistTargetConfigBuilder::binary_from_command(command, self.package)?.build();
            self.targets.get_or_insert(vec![]).push(binary);
            Ok(self)
        }
    }

    feature_command_dist_build_license! {
        /// Adds a package license files to the list of files to be distributed.
        pub fn license_files(mut self, files: impl IntoIterator<Item = Utf8PathBuf>) -> Self {
            self.license_files = Some(files.into_iter().collect());
            self
        }
    }

    feature_command_dist_build_doc! {
        /// Adds a package documentation files to the list of files to be distributed.
        pub fn documents(mut self, files: impl IntoIterator<Item = Utf8PathBuf>) -> Self {
            self.documents = Some(files.into_iter().collect());
            self
        }
    }

    /// Builds a [`DistPackageConfig`] from the current configuration.
    pub fn build(self) -> DistPackageConfig<'a> {
        DistPackageConfig {
            name: self.name,
            package: self.package,
            targets: self.targets,
            #[cfg(feature = "command-dist-build-license")]
            license_files: self.license_files,
            #[cfg(feature = "command-dist-build-doc")]
            documents: self.documents,
        }
    }
}

/// Configuration for the distribution of the package.
#[derive(Debug)]
pub struct DistPackageConfig<'a> {
    name: String,
    package: &'a Package,
    targets: Option<Vec<DistTargetConfig<'a>>>,
    #[cfg(feature = "command-dist-build-license")]
    license_files: Option<Vec<Utf8PathBuf>>,
    #[cfg(feature = "command-dist-build-doc")]
    documents: Option<Vec<Utf8PathBuf>>,
}

impl<'a> DistPackageConfig<'a> {
    /// Returns the name of the package.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata of the package.
    pub fn package(&self) -> &'a Package {
        self.package
    }

    /// Returns the list of targets to be distributed.
    pub fn targets(&self) -> Option<&[DistTargetConfig]> {
        self.targets.as_deref()
    }

    /// Returns the path to the package's root directory.
    pub fn root_dir(&self) -> &Utf8Path {
        self.package.manifest_path.parent().unwrap()
    }

    feature_command_dist_build_license! {
        /// Returns the list of license files to be distributed.
        pub fn license_files(&self) -> Option<&[Utf8PathBuf]> {
            self.license_files.as_deref()
        }
    }

    feature_command_dist_build_doc! {
        /// Returns the list of documentation files to be distributed.
        pub fn documents(&self) -> Option<&[Utf8PathBuf]> {
            self.documents.as_deref()
        }
    }
}
