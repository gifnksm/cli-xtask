use clap::Parser;
use cli_xtask::{process, workspace, Config};

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "lint", skip_all, err)]
    pub(crate) fn run(&self, config: &Config) -> eyre::Result<()> {
        let Self {} = self;

        // cargo fmt --check
        cli_xtask::command::Fmt::parse_from(["fmt", "--", "--check"]).run(config)?;
        // cargo clippy -- -D warnings
        cli_xtask::command::Clippy::parse_from(["clippy", "--", "--", "-D", "warnings"])
            .run(config)?;
        // cargo rdme --check
        cli_xtask::command::Rdme::parse_from(["rdme", "--", "--check"]).run(config)?;
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
