use cli_xtask::workspace;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Arguments to pass to the `cargo test`
    extra_options: Vec<String>,
}

impl Args {
    #[tracing::instrument(name = "test", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                for feature_args in crate::feature_combinations(package) {
                    // cargo test --package <pkg> <features> <extra_options>
                    // DO NOT USE `--all-targets` here, doctests are not built with `--all-targets`
                    crate::execute_on(
                        metadata,
                        "cargo",
                        ["test", "--package", &package.name]
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
