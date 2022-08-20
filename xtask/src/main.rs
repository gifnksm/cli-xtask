use cli_xtask::{args::GenericArgs, clap, config::Config, Result, Run};

mod xtask_test;

#[derive(Debug, clap::Subcommand)]
enum Command {
    #[clap(flatten)]
    Command(cli_xtask::command::Command),
    XtaskTest(xtask_test::XtaskTest),
}

impl Run for Command {
    fn run(&self, config: &Config) -> Result<()> {
        match self {
            Command::Command(command) => command.run(config)?,
            Command::XtaskTest(xtask_test) => xtask_test.run(config)?,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    GenericArgs::<Command>::main()
}
