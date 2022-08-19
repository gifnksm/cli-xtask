use app::Args;
use cli_xtask::{
    clap::{CommandFactory, Parser},
    config::{ConfigBuilder, DistConfigBuilder},
};

fn main() -> cli_xtask::Result<()> {
    let args = cli_xtask::args::Args::parse();

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
