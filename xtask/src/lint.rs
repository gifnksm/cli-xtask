use cli_xtask::{config::Config, subcommand::Lint, tracing, Result};

#[tracing::instrument(name = "lint", skip_all, err)]
pub fn run(args: &Lint, config: &Config) -> Result<()> {
    let mut subcommands = args.subcommands();
    subcommands.push(Box::new(crate::lint_doc::LintDoc {}));

    for subcommand in subcommands {
        subcommand.run(config)?;
    }

    Ok(())
}
