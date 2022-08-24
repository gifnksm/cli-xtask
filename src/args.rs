//! Data structures for command line arguments parsing.

use std::{env, iter};

use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package};
use eyre::eyre;
use tracing::Level;

use crate::{
    workspace::{self, FeatureOption, MetadataExt, PackageExt},
    Result,
};

/// Commmand line arguments to control log verbosity level.
///
/// # Examples
///
/// To get `--quiet` (`-q`) and `--verbose` (or `-v`) flags through your entire
/// program, just `flattern` this struct:
///
/// ```rust
/// use cli_xtask::{args::Verbosity, clap};
///
/// #[derive(Debug, clap::Parser)]
/// struct App {
///     #[clap(flatten)]
///     verbosity: Verbosity,
/// }
/// ```
///
/// The [`LogLevel`](crate::tracing::Level) values returned by
/// [`Verbosity::get()`](crate::args::Verbosity::get) are:
///
/// * `None`: `-qqq`
/// * `Some(Level::ERROR)`: `-qq`
/// * `Some(Level::WARN)`: `-q`
/// * `Some(Level::INFO)`: no arguments
/// * `Some(Level::DEBUG)`: `-v`
/// * `Some(Level::TRACE)`: `-vv`
#[derive(Debug, Clone, Default, clap::Args)]
pub struct Verbosity {
    /// More output per occurrence
    #[clap(long, short = 'v', parse(from_occurrences), global = true)]
    verbose: i8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        parse(from_occurrences),
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: i8,
}

impl Verbosity {
    /// Returns the log verbosity level.
    pub fn get(&self) -> Option<Level> {
        let level = self.verbose - self.quiet;
        match level {
            i8::MIN..=-3 => None,
            -2 => Some(Level::ERROR),
            -1 => Some(Level::WARN),
            0 => Some(Level::INFO),
            1 => Some(Level::DEBUG),
            2..=i8::MAX => Some(Level::TRACE),
        }
    }
}

/// Command line arguments to specify the environment variables to set for the
/// subcommand.
#[derive(Debug, Clone, Default, clap::Args)]
pub struct EnvArgs {
    /// Environment variables to set for the subcommand.
    #[clap(
        long,
        short = 'e',
        value_name = "KEY>=<VALUE", // hack
        parse(from_str = EnvArgs::parse_parts),
    )]
    pub env: Vec<(String, String)>,
}

impl EnvArgs {
    /// Creates a new `EnvArgs` from an iterator of `(key, value)` pairs.
    pub fn new(iter: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        Self {
            env: iter
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    fn parse_parts(s: &str) -> (String, String) {
        match s.split_once('=') {
            Some((key, value)) => (key.into(), value.into()),
            None => (s.into(), "".into()),
        }
    }
}

/// Command line arguments to specify the workspaces where the subcommand runs
/// on.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct WorkspaceArgs {
    /// Same as `--all-workspaces --workspace --each-feature`.
    #[clap(long)]
    pub exhaustive: bool,
    /// Run the subcommand on all workspaces.
    #[clap(long, conflicts_with = "exhaustive")]
    pub all_workspaces: bool,
    /// Run the subcommand on each workspace other than the current workspace.
    #[clap(long)]
    pub exclude_current_workspace: bool,
}

impl WorkspaceArgs {
    /// `WorkspaceArgs` value with `--exhaustive` flag enabled.
    pub const EXHAUSTIVE: Self = Self {
        exhaustive: true,
        all_workspaces: false,
        exclude_current_workspace: false,
    };

    /// Returns the workspaces to run the subcommand on.
    pub fn workspaces(&self) -> impl Iterator<Item = &'static Metadata> {
        let workspaces = if self.exhaustive || self.all_workspaces {
            if self.exclude_current_workspace {
                &workspace::all()[1..]
            } else {
                workspace::all()
            }
        } else if self.exclude_current_workspace {
            &workspace::all()[..0]
        } else {
            &workspace::all()[..1]
        };
        workspaces.iter()
    }
}

/// Command line arguments to specify the packages to run the subcommand for.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct PackageArgs {
    /// Command line arguments to specify the workspaces where the subcommand
    /// runs on.
    #[clap(flatten)]
    pub workspace_args: WorkspaceArgs,
    /// Run the subcommand for all packages in the workspace
    #[clap(long, conflicts_with = "exhaustive")]
    pub workspace: bool,
    /// Package name to run the subcommand for
    #[clap(long = "package", short = 'p', conflicts_with = "exhaustive")]
    pub package: Option<String>,
}

impl PackageArgs {
    /// `PackageArgs` value with `--exhaustive` flag enabled.
    pub const EXHAUSTIVE: Self = Self {
        workspace_args: WorkspaceArgs::EXHAUSTIVE,
        workspace: false,
        package: None,
    };

    /// Returns the packages to run the subcommand on.
    pub fn packages(
        &self,
    ) -> impl Iterator<Item = Result<(&'static Metadata, &'static Package)>> + '_ {
        self.workspace_args
            .workspaces()
            .map(move |workspace| {
                let packages = if self.workspace_args.exhaustive || self.workspace {
                    workspace.workspace_packages()
                } else if let Some(name) = &self.package {
                    let pkg = workspace
                        .workspace_package_by_name(name)
                        .ok_or_else(|| eyre!("Package not found"))?;
                    vec![pkg]
                } else {
                    let current_dir = Utf8PathBuf::try_from(env::current_dir()?)?;
                    let pkg = workspace
                        .workspace_package_by_path(&current_dir)
                        .or_else(|| workspace.root_package())
                        .ok_or_else(|| eyre!("Package not found"))?;
                    vec![pkg]
                };
                let it = packages
                    .into_iter()
                    .map(move |package| (workspace, package));
                Ok(it)
            })
            .flat_map(|res| -> Box<dyn Iterator<Item = _>> {
                match res {
                    Ok(it) => Box::new(it.map(Ok)),
                    Err(err) => Box::new(iter::once(Err(err))),
                }
            })
    }
}

/// Command line arguments to specify the features to run the subcommand with.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct FeatureArgs {
    /// Command line arguments to specify the packages to run the subcommand
    /// for.
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Run the subcommand with each feature enabled
    #[clap(long, conflicts_with = "exhaustive")]
    pub each_feature: bool,
}

impl FeatureArgs {
    /// `FeatureArgs` value with `--exhaustive` flag enabled.
    pub const EXHAUSTIVE: Self = Self {
        package_args: PackageArgs::EXHAUSTIVE,
        each_feature: false,
    };

    /// Returns the features to run the subcommand with.
    pub fn features(
        &self,
    ) -> impl Iterator<
        Item = Result<(
            &'static Metadata,
            &'static Package,
            Option<FeatureOption<'static>>,
        )>,
    > + '_ {
        self.package_args
            .packages()
            .map(move |res| {
                res.map(move |(workspace, package)| -> Box<dyn Iterator<Item = _>> {
                    let exhaustive = self.package_args.workspace_args.exhaustive;
                    if (exhaustive || self.each_feature) && !package.features.is_empty() {
                        Box::new(
                            package
                                .each_feature()
                                .map(move |feature| (workspace, package, Some(feature))),
                        )
                    } else {
                        Box::new(iter::once((workspace, package, None)))
                    }
                })
            })
            .flat_map(|res| -> Box<dyn Iterator<Item = _>> {
                match res {
                    Ok(it) => Box::new(it.map(Ok)),
                    Err(err) => Box::new(iter::once(Err(err))),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbosity() {
        use clap::Parser;
        #[derive(Debug, clap::Parser)]
        struct App {
            #[clap(flatten)]
            verbosity: Verbosity,
        }

        let cases: &[(&[&str], Option<Level>)] = &[
            (&["-qqqq"], None),
            (&["-qqq"], None),
            (&["-qq"], Some(Level::ERROR)),
            (&["-q"], Some(Level::WARN)),
            (&[], Some(Level::INFO)),
            (&["-v"], Some(Level::DEBUG)),
            (&["-vv"], Some(Level::TRACE)),
        ];

        for (arg, level) in cases {
            let args = App::parse_from(["app"].into_iter().chain(arg.iter().copied()));
            assert_eq!(args.verbosity.get(), *level, "arg: {}", arg.join(" "));
        }
    }
}
