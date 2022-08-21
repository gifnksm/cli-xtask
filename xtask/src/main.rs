use cli_xtask::{clap, config::Config, Result, Run, Xtask};

mod pre_release;
mod xtask_test;

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    #[clap(flatten)]
    Predefined(cli_xtask::subcommand::Subcommand),
    XtaskTest(xtask_test::XtaskTest),
    PreRelease(pre_release::PreRelease),
}

impl Run for Subcommand {
    fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::Predefined(args) => args.run(config)?,
            Self::XtaskTest(args) => args.run(config)?,
            Self::PreRelease(args) => args.run(config)?,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Xtask::<Subcommand>::main()
}
