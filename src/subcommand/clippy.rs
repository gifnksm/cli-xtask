use std::process::Command;

use crate::{args::FeatureArgs, config::Config, process::CommandExt, Result, Run};

/// `clippy` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Clippy {
    /// Features to run the `cargo clippy` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo clippy`
    pub extra_options: Vec<String>,
}

impl Run for Clippy {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Clippy {
    /// Runs `clippy` subcommand workflow.
    #[tracing::instrument(name = "clippy", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            Command::new("cargo")
                .args(
                    ["clippy", "--package", &package.name]
                        .into_iter()
                        .chain(features.map(|f| f.to_args()).unwrap_or_default())
                        .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
