use cargo_metadata::{camino::Utf8Path, Metadata, Package};

use super::PackageExt;

/// Extension methods for [`cargo_metadata::Metadata`].
pub trait MetadataExt {
    /// Returns the workspace package with given name.
    fn workspace_package_by_name<'a>(&'a self, name: &str) -> Option<&'a Package>;

    /// Returns the workspace package with given path.
    ///
    /// `path` must be absolute and normalized.
    fn workspace_package_by_path(&'_ self, path: impl AsRef<Utf8Path>) -> Option<&'_ Package>;
}

impl MetadataExt for Metadata {
    fn workspace_package_by_name<'a>(&'a self, name: &str) -> Option<&'a Package> {
        self.workspace_packages()
            .into_iter()
            .find(|p| p.name == name)
    }

    fn workspace_package_by_path(&'_ self, path: impl AsRef<Utf8Path>) -> Option<&'_ Package> {
        let path = path.as_ref();
        assert!(path.is_absolute());
        self.workspace_packages()
            .into_iter()
            .filter(|package| path.starts_with(package.root_directory()))
            .max_by_key(|p| p.manifest_path.as_str().len())
    }
}
