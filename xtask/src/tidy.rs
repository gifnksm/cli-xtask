use cli_xtask::{config::Config, subcommand::Tidy, tracing, Result};

#[tracing::instrument(name = "tidy", parent = None, skip_all, err)]
pub fn run(args: &Tidy, config: &Config) -> Result<()> {
    args.run(config)?;

    crate::tidy_doc::TidyDoc {}.run(config)?;

    Ok(())
}
