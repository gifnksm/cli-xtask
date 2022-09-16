use crate::{args::Verbosity, config::Config, Result, Run};

#[cfg(feature = "main")]
#[cfg_attr(docsrs, doc(cfg(feature = "main")))]
mod main;

/// Command line interface definition for cargo xtask command.
#[cfg_attr(doc, doc = include_str!("../doc/cargo-xtask.md"))]
///
/// # Examples
///
/// Use the premade entry point function with default configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(feature = "main")]
/// # {
/// use cli_xtask::{Result, Xtask};
///
/// fn main() -> Result<()> {
///     <Xtask>::main()
/// }
/// # }
/// ```
///
/// Use the premade entry point function and custom configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(feature = "main")]
/// # {
/// use cli_xtask::{config::Config, Result, Xtask};
///
/// fn main() -> Result<()> {
///     <Xtask>::main_with_config(|| Ok(Config::new()))
/// }
/// # }
/// ```
///
/// If you don't want to use the `main` feature, write the main function as
/// follows:
///
/// ```rust
/// # #[cfg(all(feature = "error-handler", feature = "logger"))]
/// # {
/// use cli_xtask::{clap::Parser, config::Config, error_handler, logger, Result, Xtask};
///
/// fn main() -> Result<()> {
///     // Parse command line arguments
///     let command = <Xtask>::parse();
///
///     // Setup error handler and logger
///     error_handler::install()?; // `error-handler` feature is required
///     logger::install(command.verbosity.get())?; // `logger` feature is required
///
///     // Run the subcommand specified by the command line arguments
///     command.run(&Config::new())?;
///
///     Ok(())
/// }
/// # }
/// ```
///
/// If you want to define your own subcommands, declare the type that implements
/// [`clap::Subcommand`] and [`Run`], then use `Xtask<YourOwnSubcommand>`
/// instead of `Xtask`.
///
/// ```rust
/// # #[cfg(feature = "main")]
/// # {
/// use cli_xtask::{
///     clap::{self, Parser},
///     config::Config,
///     subcommand, Result, Run, SubcommandRun, Xtask,
/// };
///
/// // Define your own subcommand arguments
/// #[derive(Debug, clap::Subcommand)]
/// enum YourOwnSubcommand {
///     #[clap(flatten)]
///     Predefined(subcommand::Subcommand),
///     /// Run foo function.
///     Foo,
///     /// Run bar function
///     Bar,
/// }
///
/// impl Run for YourOwnSubcommand {
///     fn run(&self, config: &Config) -> Result<()> {
///         match self {
///             Self::Predefined(subcommand) => subcommand.run(config)?,
///             Self::Foo => println!("foo!"),
///             Self::Bar => println!("bar!"),
///         }
///         Ok(())
///     }
///
///     fn to_subcommands(&self) -> Option<SubcommandRun> {
///         None
///     }
///
///     fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
///         self
///     }
///
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
///
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
///         self
///     }
/// }
///
/// fn main() -> Result<()> {
///     Xtask::<YourOwnSubcommand>::main()
/// }
/// # }
/// ```
#[derive(Debug, Clone, Default, clap::Parser)]
#[non_exhaustive]
#[clap(bin_name = "cargo xtask", about = "Rust project automation command", long_about = None)]
pub struct Xtask<Subcommand = crate::subcommand::Subcommand>
where
    Subcommand: clap::Subcommand,
{
    /// Verbosity level
    #[clap(flatten)]
    pub verbosity: Verbosity,

    /// Subcommand to run
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

impl<Subcommand> Xtask<Subcommand>
where
    Subcommand: clap::Subcommand,
{
    /// Runs the subcommand specified by the command line arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(all(feature = "error-handler", feature = "logger"))]
    /// # {
    /// use cli_xtask::{clap::Parser, config::Config, error_handler, logger, Result, Xtask};
    ///
    /// fn main() -> Result<()> {
    ///     // Parse command line arguments
    ///     let command = <Xtask>::parse();
    ///
    ///     // Setup error handler and logger
    ///     error_handler::install()?; // `error-handler` feature is required
    ///     logger::install(command.verbosity.get())?; // `logger` feature is required
    ///
    ///     // Run the subcommand specified by the command line arguments
    ///     command.run(&Config::new())?;
    ///
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn run(&self, config: &Config) -> Result<()>
    where
        Subcommand: Run,
    {
        let _config = config; // suppress unused-var warnings

        match &self.subcommand {
            Some(command) => command.run(config)?,
            None => <Self as clap::CommandFactory>::command().print_help()?,
        }

        Ok(())
    }
}
