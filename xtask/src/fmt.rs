use cli_xtask::{process, workspace};

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    /// Arguments to pass to the `cargo fmt`
    extra_options: Vec<String>,
}

impl Args {
    #[tracing::instrument(name = "fmt", skip_all, err)]
    pub(crate) fn run(&self) -> eyre::Result<()> {
        let Self { extra_options } = self;

        for metadata in workspace::all() {
            for package in metadata.workspace_packages() {
                process::execute_on(
                    metadata,
                    "cargo",
                    ["fmt", "--package", &package.name]
                        .into_iter()
                        .chain(extra_options.iter().map(String::as_str)),
                )?;
            }
        }

        Ok(())
    }
}
