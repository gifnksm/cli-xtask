use clap::Parser;
use cli_xtask::{process, workspace};

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "lint", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self {} = self;

        // cargo fmt --check
        crate::fmt::Args::parse_from(["fmt", "--", "--check"]).run()?;
        // cargo clippy -- -D warnings
        crate::clippy::Args::parse_from(["clippy", "--", "--", "-D", "warnings"]).run()?;
        // cargo rdme --check
        crate::rdme::Args::parse_from(["rdme", "--", "--check"]).run()?;
        // cargo udeps
        crate::udeps::Args::parse_from(["udeps"]).run()?;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                for feature_args in crate::feature_combinations(package) {
                    // rustup run nightly cargo udeps --package <pkg> <features>
                    // cargo +nightly udeps fails on windows, so use rustup instead
                    process::execute_on(
                        metadata,
                        "rustup",
                        [
                            "run",
                            "nightly",
                            "cargo",
                            "udeps",
                            "--package",
                            &package.name,
                        ]
                        .into_iter()
                        .chain(feature_args.iter().copied()),
                    )?;
                }
            }
        }

        Ok(())
    }
}
