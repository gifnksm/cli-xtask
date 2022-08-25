use cli_xtask::{config::Config, subcommand::Tidy, tracing, Result};

#[tracing::instrument(name = "tidy", skip_all, err)]
pub fn run(args: &Tidy, config: &Config) -> Result<()> {
    let mut subcommands = args.subcommands();
    subcommands.push(Box::new(crate::tidy_doc::TidyDoc {}));

    for subcommand in subcommands {
        subcommand.run(config)?;
    }

    Ok(())
}
