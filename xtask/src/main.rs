use cli_xtask::{clap, config::Config, subcommand::Subcommand as Predefined, Result, Run, Xtask};

mod pre_release;
mod xtask_test;

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    #[clap(flatten)]
    Predefined(Predefined),
    XtaskTest(xtask_test::XtaskTest),
}

impl Run for Subcommand {
    fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::Predefined(Predefined::PreRelease(args)) => pre_release::run(args, config)?,
            Self::Predefined(args) => args.run(config)?,
            Self::XtaskTest(args) => args.run(config)?,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Xtask::<Subcommand>::main()
}
