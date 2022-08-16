use app::Args;
use clap::CommandFactory;
use cli_xtask::{command::Command, ConfigBuilder, DistConfigBuilder};

fn main() -> eyre::Result<()> {
    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger()?;

    let metadata = cli_xtask::cargo_workspace();
    let (dist, package) = DistConfigBuilder::from_root_package(metadata)?;
    let dist = dist
        .package(package.binary_from_command(Args::command())?.build())
        .build();
    let config = ConfigBuilder::new().dist(dist).build();
    <Command as clap::Parser>::parse().run(&config)?;

    Ok(())
}
