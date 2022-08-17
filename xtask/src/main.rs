use cargo_metadata::Package;
use clap::Parser;

mod build;
mod clippy;
mod exec;
mod fmt;
mod lint;
mod rdme;
mod test;
mod udeps;

#[derive(Debug, Parser)]
enum Args {
    /// Run `cargo build` on all workspaces in the current directory and subdirectories
    Build(build::Args),
    /// Run `cargo clippy` on all workspaces in the current directory and subdirectories
    Clippy(clippy::Args),
    /// Run commands on all workspaces in the current directory and subdirectories
    Exec(exec::Args),
    /// Run `cargo fmt` on all workspaces in the current directory and subdirectories
    Fmt(fmt::Args),
    /// Run all lint commands on all workspaces in the current directory and subdirectories
    Lint(lint::Args),
    /// Run `cargo rdme` on all workspaces in the current directory and subdirectories
    Rdme(rdme::Args),
    /// Run `cargo test` on all workspaces in the current directory and subdirectories
    Test(test::Args),
    /// Run `cargo udeps` on all workspaces in the current directory and subdirectories
    Udeps(udeps::Args),
}

impl Args {
    fn run(&self) -> eyre::Result<()> {
        match self {
            Self::Build(args) => args.run(),
            Self::Clippy(args) => args.run(),
            Self::Exec(args) => args.run(),
            Self::Fmt(args) => args.run(),
            Self::Lint(args) => args.run(),
            Self::Rdme(args) => args.run(),
            Self::Test(args) => args.run(),
            Self::Udeps(args) => args.run(),
        }
    }
}

fn main() -> eyre::Result<()> {
    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger()?;

    tracing::info!("Running on {}", std::env::current_dir()?.display());
    Args::parse().run()?;

    Ok(())
}

fn feature_combinations(package: &Package) -> Vec<Vec<&str>> {
    if package.features.is_empty() {
        return vec![vec![]];
    }

    let features = package.features.keys();
    let mut args = vec![vec!["--all-features"], vec!["--no-default-features"]];
    for feature in features {
        args.push(vec!["--features", feature, "--no-default-features"]);
    }
    args
}
