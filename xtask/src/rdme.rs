use cli_xtask::workspace;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Arguments to pass to the `cargo rdme`
    extra_options: Vec<String>,
}

impl Args {
    #[tracing::instrument(name = "rdme", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            crate::execute_on(
                metadata,
                "cargo",
                ["rdme"]
                    .into_iter()
                    .chain(extra_options.iter().map(String::as_str)),
            )?;
        }

        Ok(())
    }
}
