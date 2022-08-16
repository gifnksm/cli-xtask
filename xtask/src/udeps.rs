#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Arguments to pass to the `cargo udeps`
    extra_options: Vec<String>,
}

impl Args {
    #[tracing::instrument(name = "udeps", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for (_path, metadata) in crate::all_workspaces()? {
            for package in metadata.workspace_packages() {
                for feature_args in crate::feature_combinations(package) {
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
                        .chain(feature_args.iter().copied())
                        .chain(extra_options.iter().map(String::as_str)),
                    )?;
                }
            }
        }

        Ok(())
    }
}
