use std::{collections::HashMap, fs, process::Command};

use cargo_metadata::Package;
use serde::Deserialize;

use crate::{
    args::{EnvArgs, PackageArgs},
    config::Config,
    process::CommandExt,
    Error, Result, Run,
};

/// Arguments definition of the `docsrs` subcommand.
#[cfg_attr(doc, doc = include_str!("../../doc/cargo-xtask-docsrs.md"))]
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Docsrs {
    /// Environment variables to set for `cargo doc`.
    #[clap(flatten)]
    pub env_args: EnvArgs,
    /// Packages to run the `cargo doc` with.
    #[clap(flatten)]
    pub package_args: PackageArgs,
    /// Build documents for docs.rs's default target
    #[clap(long)]
    pub default_target: bool,
    /// Build documents for all supported targets.
    #[clap(long)]
    pub all_targets: bool,
    /// Options to pass to the `cargo doc`.
    pub extra_options: Vec<String>,
}

impl Run for Docsrs {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Docsrs {
    /// Runs the `docsrs` subcommand.
    #[tracing::instrument(name = "docsrs", skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            env_args,
            package_args,
            default_target,
            all_targets,
            extra_options,
        } = self;

        for res in package_args.packages() {
            let (workspace, package) = res?;
            let metadata = DocsrsMetadata::try_from(package)?;
            let target_options = if *all_targets || *default_target {
                metadata
                    .target_options(*all_targets)
                    .into_iter()
                    .map(Some)
                    .collect::<Vec<_>>()
            } else {
                vec![None]
            };
            for target in target_options {
                // rustup run nightly cargo doc --package <pkg> <docsrs_options> <extra_options>
                // `cargo +nightly doc` fails on windows, so use rustup instead
                let mut cmd = Command::new("rustup");
                cmd.args([
                    "run",
                    "nightly",
                    "cargo",
                    "doc",
                    "--no-deps",
                    "--package",
                    &package.name,
                ]);
                if let Some(target) = target {
                    cmd.args(["--target", target]);
                }
                cmd.arg("-Zunstable-options")
                    .arg("-Zrustdoc-map")
                    .args(metadata.args())
                    .args(extra_options)
                    .envs(metadata.envs(&env_args.env))
                    .workspace_spawn(workspace)?;
            }

            if let Some(package) = workspace.root_package() {
                let index = workspace.target_directory.join("doc/index.html");
                fs::write(
                    index,
                    format!(
                        r#"<meta http-equiv="refresh" content="0; url=./{}/">"#,
                        package.name.replace('-', "_")
                    ),
                )?;
            }
        }

        Ok(())
    }
}

/// Package metadata for docs.rs
///
/// <https://docs.rs/about/metadata>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DocsrsMetadata {
    /// Features to pass to Cargo (default: [])
    #[serde(default)]
    features: Vec<String>,
    /// Whether to pass `--all-features` to Cargo (default: false)
    #[serde(default)]
    all_features: bool,
    /// Whether to pass `--no-default-features` to Cargo (default: false)
    #[serde(default)]
    no_default_features: bool,
    /// Target to test build on, used as the default landing page (default:
    /// "x86_64-unknown-linux-gnu")
    ///
    /// Any target supported by rustup can be used.
    #[serde(default)]
    default_target: Option<String>,
    /// Targets to build (default: see below)
    ///
    /// Any target supported by rustup can be used.
    ///
    /// Default targets:
    /// - x86_64-unknown-linux-gnu
    /// - x86_64-apple-darwin
    /// - x86_64-pc-windows-msvc
    /// - i686-unknown-linux-gnu
    /// - i686-pc-windows-msvc
    ///
    /// Set this to `[]` to only build the default target.
    ///
    /// # If `default-target` is unset, the first element of `targets` is treated as the default target.
    /// Otherwise, these `targets` are built in addition to the default target.
    /// If both `default-target` and `targets` are unset,
    ///   all tier-one targets will be built and `x86_64-unknown-linux-gnu` will
    /// be used as the default target.
    #[serde(default = "default_targets")]
    targets: Vec<String>,
    /// Additional `RUSTFLAGS` to set (default: [])
    #[serde(default)]
    rustc_args: Vec<String>,
    /// Additional `RUSTDOCFLAGS` to set (default: [])
    #[serde(default)]
    rustdoc_args: Vec<String>,
    /// List of command line arguments for `cargo`.
    ///
    /// These cannot be a subcommand, they may only be options.
    #[serde(default)]
    cargo_args: Vec<String>,
}

impl TryFrom<&Package> for DocsrsMetadata {
    type Error = Error;

    fn try_from(value: &Package) -> Result<Self> {
        let table = || value.metadata.get("docs")?.get("rs");
        let table = match table() {
            Some(table) => table,
            None => return Ok(Self::default()),
        };
        let metadata = serde_json::from_value(table.clone())?;
        Ok(metadata)
    }
}

fn default_targets() -> Vec<String> {
    [
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "i686-unknown-linux-gnu",
        "i686-pc-windows-msvc",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

impl DocsrsMetadata {
    fn target_options(&self, all_targets: bool) -> Vec<&str> {
        if all_targets {
            self.targets.iter().map(|s| s.as_str()).collect()
        } else {
            vec![self.default_target()]
        }
    }

    fn default_target(&self) -> &str {
        self.default_target.as_deref().unwrap_or_else(|| {
            self.targets
                .first()
                .map(|s| s.as_str())
                .unwrap_or("x86_64-unknown-linux-gnu")
        })
    }

    fn args(&self) -> Vec<&str> {
        let mut args = vec![];
        for feature in &self.features {
            args.extend(["--feature", feature]);
        }
        if self.all_features {
            args.push("--all-features");
        }
        if self.no_default_features {
            args.push("--no-default-features");
        }
        if !self.cargo_args.is_empty() {
            args.extend(self.cargo_args.iter().map(|s| s.as_str()));
        }
        args
    }

    fn envs(&self, base_env: &[(String, String)]) -> HashMap<String, String> {
        let mut envs: HashMap<String, String> = base_env.iter().cloned().collect();
        if !self.rustc_args.is_empty() {
            let s = envs.entry("RUSTFLAGS".to_string()).or_default();
            if !s.is_empty() {
                s.push(' ');
            }
            s.push_str(&self.rustc_args.join(" "));
        }

        // copied from https://github.com/rust-lang/docs.rs/blob/4635eb745e77c6de9c055cb7334f48375c0cda5d/src/docbuilder/rustwide_builder.rs#L776
        let mut rustdoc_args = vec![
            "-Zunstable-options",
            // Comment out so that static resources are loaded when the document is published on
            // GitHub Pages
            // "--static-root-path", "/-/rustdoc.static/",

            // Comment out to accept `-D warnings` for CI
            // "--cap-lints", "warn",
            "--extern-html-root-takes-precedence",
        ];
        rustdoc_args.extend(self.rustdoc_args.iter().map(|s| s.as_str()));
        let s = envs.entry("RUSTDOCFLAGS".to_string()).or_default();
        if !s.is_empty() {
            s.push(' ');
        }
        s.push_str(&rustdoc_args.join(" "));

        envs
    }
}
