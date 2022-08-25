use cli_xtask::{config::Config, subcommand::PreRelease, tracing, Result};

#[tracing::instrument(name = "pre-release", skip_all, err)]
pub fn run(args: &PreRelease, config: &Config) -> Result<()> {
    args.run(config)?;

    crate::xtask_test::XtaskTest {
        cargo_llvm_cov: false,
    }
    .run(config)?;

    Ok(())
}
