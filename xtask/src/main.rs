use cli_xtask::{clap, config::Config, subcommand::Subcommand as Predefined, Result, Run, Xtask};

mod lint;
mod lint_doc;
mod pre_release;
mod tidy;
mod tidy_doc;
mod util;
mod xtask_test;

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    #[clap(flatten)]
    Predefined(Predefined),

    LintDoc(lint_doc::LintDoc),
    TidyDoc(tidy_doc::TidyDoc),
    XtaskTest(xtask_test::XtaskTest),
}

impl Run for Subcommand {
    fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::Predefined(Predefined::PreRelease(args)) => pre_release::run(args, config)?,
            Self::Predefined(Predefined::Lint(args)) => lint::run(args, config)?,
            Self::Predefined(Predefined::Tidy(args)) => tidy::run(args, config)?,
            Self::Predefined(args) => args.run(config)?,

            Self::LintDoc(args) => args.run(config)?,
            Self::TidyDoc(args) => args.run(config)?,
            Self::XtaskTest(args) => args.run(config)?,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Xtask::<Subcommand>::main()
}
