use app::Args;
use clap::CommandFactory;
use cli_xtask::config::{ConfigBuilder, DistConfigBuilder};

fn main() -> eyre::Result<()> {
    let args = <cli_xtask::args::Args as clap::Parser>::parse();

    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger(args.verbosity())?;

    let metadata = cli_xtask::workspace::current();
    let (dist, package) = DistConfigBuilder::from_root_package(metadata)?;
    let dist = dist
        .package(package.binary_from_command(Args::command())?.build())
        .build();
    let config = ConfigBuilder::new().dist(dist).build();
    args.run(&config)?;

    Ok(())
}
