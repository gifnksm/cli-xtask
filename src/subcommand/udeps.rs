use std::process::Command;

use crate::{args::FeatureArgs, config::Config, process::CommandExt, Result, Run};

/// `udeps` subcommand arguments.
#[derive(Debug, Clone, Default, clap::Args)]
#[non_exhaustive]
pub struct Udeps {
    /// Features to run the `cargo udeps` with
    #[clap(flatten)]
    pub feature_args: FeatureArgs,
    /// Options to pass to the `cargo udeps`
    pub extra_options: Vec<String>,
}

impl Run for Udeps {
    fn run(&self, config: &Config) -> Result<()> {
        self.run(config)
    }
}

impl Udeps {
    /// Execute `udeps` subcommand workflow.
    #[tracing::instrument(name = "udeps", parent = None, skip_all, err)]
    pub fn run(&self, _config: &Config) -> Result<()> {
        let Self {
            feature_args,
            extra_options,
        } = self;

        for res in feature_args.features() {
            let (workspace, package, features) = res?;
            // rustup run nightly cargo udeps --package <pkg> <features> <extra_options>
            // `cargo +nightly udeps` fails on windows, so use rustup instead
            Command::new("rustup")
                .args(
                    [
                        "run",
                        "nightly",
                        "cargo",
                        "udeps",
                        "--package",
                        &package.name,
                    ]
                    .into_iter()
                    .chain(features.map(|f| f.to_args()).unwrap_or_default())
                    .chain(extra_options.iter().map(String::as_str)),
                )
                .workspace_spawn(workspace)?;
        }

        Ok(())
    }
}
