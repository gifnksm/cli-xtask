use cli_xtask::{config::Config, subcommand::Lint, tracing, Result};

#[tracing::instrument(name = "lint", parent = None, skip_all, err)]
pub fn run(args: &Lint, config: &Config) -> Result<()> {
    args.run(config)?;

    crate::lint_doc::LintDoc {}.run(config)?;

    Ok(())
}
