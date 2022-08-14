use clap::CommandFactory;
use cli_xtask::command::Command;
use with_command::Args;

fn main() -> eyre::Result<()> {
    cli_xtask::install_error_handler()?;
    cli_xtask::install_logger()?;

    let metadata = cli_xtask::get_metadata();
    let (dist, package) = cli_xtask::DistConfigBuilder::from_root_package(metadata)?;
    let dist = dist
        .package(package.binary_from_command(Args::command())?.build())
        .build();
    <Command as clap::Parser>::parse().run(&dist)?;

    Ok(())
}
