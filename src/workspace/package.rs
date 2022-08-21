use cargo_metadata::{camino::Utf8Path, Package};

/// Extension methods for [`cargo_metadata::Package`].
pub trait PackageExt {
    /// Returns the iterator over each feature options for the package.
    fn each_feature(&'_ self) -> EachFeature<'_>;

    /// Returns the package root directory.
    fn root_directory(&self) -> &Utf8Path;
}

impl PackageExt for Package {
    fn each_feature(&'_ self) -> EachFeature<'_> {
        let mut features = self.features.keys().collect::<Vec<_>>();
        features.sort();
        EachFeature {
            all_features: true,
            no_default_features: true,
            features: features.into_iter(),
        }
    }

    fn root_directory(&self) -> &Utf8Path {
        // `manifest_path` is the path to the manifest file, so parent must exist.
        self.manifest_path.parent().unwrap()
    }
}

/// Iterator over each feature options for the package.
///
/// This iterator is created by [`PackageExt::each_feature`].
#[derive(Debug)]
pub struct EachFeature<'a> {
    all_features: bool,
    features: std::vec::IntoIter<&'a String>,
    no_default_features: bool,
}

impl<'a> Iterator for EachFeature<'a> {
    type Item = FeatureOption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.all_features {
            self.all_features = false;
            return Some(FeatureOption::AllFeatures);
        }
        if self.no_default_features {
            self.no_default_features = false;
            return Some(FeatureOption::NoDefaultFeatures);
        }
        if let Some(feature) = self.features.next() {
            return Some(FeatureOption::Features(vec![feature]));
        }
        None
    }
}

/// Feature option for the package.
#[derive(Debug, Clone)]
pub enum FeatureOption<'a> {
    /// `--all-features`
    AllFeatures,
    /// `--no-default-features`
    NoDefaultFeatures,
    /// `--features <feature> --no-default-features`
    Features(Vec<&'a str>),
}

impl<'a> FeatureOption<'a> {
    /// Convert the value to corresponding cargo option strings.
    pub fn to_args(&self) -> Vec<&'a str> {
        match self {
            Self::AllFeatures => vec!["--all-features"],
            Self::NoDefaultFeatures => vec!["--no-default-features"],
            Self::Features(features) => {
                let mut args = Vec::with_capacity(features.len() * 2 + 1);
                for feature in features {
                    args.extend(["--features", feature]);
                }
                args.push("--no-default-features");
                args
            }
        }
    }
}
