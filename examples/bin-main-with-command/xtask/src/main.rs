use app::Args as AppArgs;
use cli_xtask::{
    args::Args,
    clap::CommandFactory,
    config::{ConfigBuilder, DistConfigBuilder},
    workspace, Result,
};

fn main() -> Result<()> {
    Args::main_with_config(|| {
        let workspace = workspace::current();
        let (dist, package) = DistConfigBuilder::from_root_package(workspace)?;
        let command = AppArgs::command();
        let target = package
            .binary_by_name(command.get_name())?
            .command(command)
            .build()?;
        let dist = dist.package(package.target(target).build()?).build()?;
        let config = ConfigBuilder::new().dist(dist).build()?;
        Ok(config)
    })
}
