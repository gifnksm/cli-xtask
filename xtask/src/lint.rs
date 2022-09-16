use cli_xtask::{config::Config, subcommand::Lint, tracing, Result};

#[tracing::instrument(name = "lint", skip_all, err)]
pub fn run(args: &Lint, config: &Config) -> Result<()> {
    let mut subcommands = args.to_subcommands();
    subcommands
        .subcommands_mut()
        .push(Box::new(crate::lint_doc::LintDoc {}));
    subcommands.run(config)
}
