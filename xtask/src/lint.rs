#[derive(Debug, clap::Parser)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "lint", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self {} = self;

        for (_path, metadata) in crate::all_workspaces()? {
            // cargo fmt --all --check
            crate::execute_on(&metadata, "cargo", ["fmt", "--all", "--check"])?;

            for package in metadata.workspace_packages() {
                for feature_args in crate::feature_combinations(package) {
                    // rustup run nightly cargo udeps --package <pkg> <features>
                    // cargo +nightly udeps fails on windows, so use rustup instead
                    crate::execute_on(
                        &metadata,
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

                    // cargo clippy --all-targets --package <pkg> <features> -- -D warnings
                    crate::execute_on(
                        &metadata,
                        "cargo",
                        ["clippy", "--all-targets", "--package", &package.name]
                            .into_iter()
                            .chain(feature_args.iter().copied())
                            .chain(["--", "-D", "warnings"]),
                    )?;
                }
            }
        }

        Ok(())
    }
}
