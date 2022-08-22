use crate::{
    config::{Config, ConfigBuilder, DistConfigBuilder, DistTargetConfigBuilder},
    workspace, Result, Run, Xtask,
};

impl<Subcommand> Xtask<Subcommand>
where
    Subcommand: clap::Subcommand + Run,
{
    /// Entry point for xtask crate.
    ///
    /// This function initializes error handler and logger, then runs the
    /// subcommand. Default configuration will be passed to subcommand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cli_xtask::{Result, Xtask};
    ///
    /// fn main() -> Result<()> {
    ///     <Xtask>::main()
    /// }
    /// ```
    pub fn main() -> Result<()> {
        Self::main_with_config(|| {
            let workspace = workspace::current();
            let (dist, package) = DistConfigBuilder::from_root_package(workspace)?;
            let targets = package
                .all_binaries()
                .into_iter()
                .map(DistTargetConfigBuilder::build)
                .collect::<Result<Vec<_>>>()?;
            let package = package.targets(targets).build()?;
            let dist = dist.package(package).build()?;
            let config = ConfigBuilder::new().dist(dist).build()?;
            Ok(config)
        })
    }

    /// Entry point for xtask crate.
    ///
    /// This function initializes error handler and logger, then runs the
    /// subcommand. Generated configuration by `config` argument will be
    /// passed to subcommand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cli_xtask::{config::Config, Result, Xtask};
    ///
    /// fn main() -> Result<()> {
    ///     <Xtask>::main_with_config(|| Ok(Config::new()))
    /// }
    /// ```
    pub fn main_with_config<'a>(config: impl FnOnce() -> Result<Config<'a>>) -> Result<()> {
        let args = <Self as clap::Parser>::parse();

        crate::error_handler::install()?;
        crate::logger::install(args.verbosity.get())?;

        args.run(&config()?)?;

        Ok(())
    }
}
