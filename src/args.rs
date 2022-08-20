//! Data structures for command line arguments parsing.
//!
//! If you want to use the subcomamnds of cli-xtask as-is, see the
//! [`Args`](crate::args::Args) type for more details. If you want to use the
//! subcomamnds of cli-xtask with your own arguments, see the
//! [`GenericArgs`](crate::args::GenericArgs) struct for more details.
//!
//! # Examples
//!
//! Use the premade entry point function with default configuration (`main`
//! feature is required):
//!
//! ```rust
//! # #[cfg(all(command, feature="main"))]
//! # {
//! use cli_xtask::{args::Args, Result};
//!
//! fn main() -> Result<()> {
//!     Args::main()
//! }
//! # }
//! ```
//!
//! Use the premade entry point function and custom configuration (`main`
//! feature is required):
//!
//! ```rust
//! # #[cfg(all(command, feature = "main"))]
//! # {
//! use cli_xtask::{args::Args, config::Config, Result};
//!
//! fn main() -> Result<()> {
//!     Args::main_with_config(|| Ok(Config::new()))
//! }
//! # }
//! ```
//!
//! If you don't want to use the `main` feature, write the main function as
//! follows:
//!
//! ```rust
//! # #[cfg(all(command, feature = "error-handler", feature = "logger"))]
//! # {
//! use cli_xtask::{args::Args, clap::Parser, config::Config, error_handler, logger, Result};
//!
//! fn main() -> Result<()> {
//!     // Parse command line arguments
//!     let args = Args::parse();
//!
//!     // Setup error handler and logger
//!     error_handler::install()?; // `error-handler` feature is required
//!     logger::install(args.verbosity())?; // `logger` feature is required
//!
//!     // Run the subcommand specified by the command line arguments
//!     args.run(&Config::new())?;
//!
//!     Ok(())
//! }
//! # }
//! ```

use tracing::Level;

use crate::{config::Config, Result, Run};

/// Command line interface definition.
///
/// If you want to use the subcomamnds of cli-xtask as-is, use this type.
/// If you want to use the subcomamnds of cli-xtask with your own arguments, use
/// [`GenericArgs`](crate::args::GenericArgs).
///
/// # Examples
///
/// Use the premade entry point function with default configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(all(command, feature="main"))]
/// # {
/// use cli_xtask::{args::Args, Result};
///
/// fn main() -> Result<()> {
///     Args::main()
/// }
/// # }
/// ```
///
/// Use the premade entry point function and custom configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(all(command, feature = "main"))]
/// # {
/// use cli_xtask::{args::Args, config::Config, Result};
///
/// fn main() -> Result<()> {
///     Args::main_with_config(|| Ok(Config::new()))
/// }
/// # }
/// ```
///
/// If you don't want to use the `main` feature, write the main function as
/// follows:
///
/// ```rust
/// # #[cfg(all(command, feature = "error-handler", feature = "logger"))]
/// # {
/// use cli_xtask::{args::Args, clap::Parser, config::Config, error_handler, logger, Result};
///
/// fn main() -> Result<()> {
///     // Parse command line arguments
///     let args = Args::parse();
///
///     // Setup error handler and logger
///     error_handler::install()?; // `error-handler` feature is required
///     logger::install(args.verbosity())?; // `logger` feature is required
///
///     // Run the subcommand specified by the command line arguments
///     args.run(&Config::new())?;
///
///     Ok(())
/// }
/// # }
/// ```
#[cfg(command)]
#[cfg_attr(docsrs, doc(cfg(feature = "command-*")))]
pub type Args = GenericArgs<crate::command::Command>;

/// Generic command line interface definition.
///
/// If you want to use the subcomamnds of cli-xtask as-is, use
/// [`Args`](crate::args::Args). If you want to use the subcomamnds of cli-xtask
/// with your own arguments, use this type.
///
/// # Examples
///
/// Use the premade entry point function with default configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(all(command, feature = "main"))]
/// # {
/// use cli_xtask::{
///     args::GenericArgs,
///     clap::{self, Parser},
///     command,
///     config::Config,
///     Result, Run,
/// };
///
/// // Define your own subcommand arguments
/// #[derive(Debug, clap::Subcommand)]
/// enum YourOwnCommand {
///     #[clap(flatten)]
///     Command(command::Command),
///     /// Run foo function.
///     Foo,
///     /// Run bar function
///     Bar,
/// }
///
/// impl Run for YourOwnCommand {
///     fn run(&self, config: &Config) -> Result<()> {
///         match self {
///             Self::Command(command) => command.run(config)?,
///             Self::Foo => println!("foo!"),
///             Self::Bar => println!("bar!"),
///         }
///         Ok(())
///     }
/// }
///
/// fn main() -> Result<()> {
///     GenericArgs::<YourOwnCommand>::main()
/// }
/// # }
/// ```
///
/// Use the premade entry point function and custom configuration (`main`
/// feature is required):
///
/// ```rust
/// # #[cfg(all(command, feature = "main"))]
/// # {
/// use cli_xtask::{args::GenericArgs, config::Config, Result};
///
/// // Denifition of `YourOwnCommand` is omitted.
///
/// fn main() -> Result<()> {
///     GenericArgs::<YourOwnCommand>::main_with_config(|| Ok(Config::new()))
/// }
/// # #[derive(Debug, clap::Subcommand)]
/// # enum YourOwnCommand{}
/// # impl cli_xtask::Run for YourOwnCommand {
/// #   fn run(&self, _config: &Config) -> Result<()> {
/// #       Ok(())
/// #   }
/// # }
/// # }
/// ```
///
/// If you don't want to use the `main` feature, write the main function as
/// follows:
///
/// ```rust
/// # #[cfg(all(command, feature = "error-handler", feature = "logger"))]
/// # {
/// use cli_xtask::{
///     args::GenericArgs, clap::Parser, config::Config, error_handler, logger, Result,
/// };
///
/// // Denifition of `YourOwnCommand` is omitted.
///
/// fn main() -> Result<()> {
///     // Parse command line arguments
///     let args = GenericArgs::<YourOwnCommand>::parse();
///
///     // Setup error handler and logger
///     error_handler::install()?; // `error-handler` feature is required
///     logger::install(args.verbosity())?; // `logger` feature is required
///
///     // Run the subcommand specified by the command line arguments
///     args.run(&Config::new())?;
///
///     Ok(())
/// }
/// # #[derive(Debug, clap::Subcommand)]
/// # enum YourOwnCommand {}
/// # impl cli_xtask::Run for YourOwnCommand {
/// #   fn run(&self, _config: &Config) -> Result<()> {
/// #       Ok(())
/// #   }
/// # }
/// # }
/// ```
#[derive(Debug, clap::Parser)]
#[clap(bin_name = "cargo xtask", about = "Rust project automation command", long_about = None)]
pub struct GenericArgs<Command>
where
    Command: clap::Subcommand,
{
    #[clap(flatten)]
    verbosity: Verbosity,

    #[clap(subcommand)]
    command: Option<Command>,
}

impl<Command> GenericArgs<Command>
where
    Command: clap::Subcommand,
{
    /// Runs the subcommand specified by the command line arguments.
    pub fn run(&self, config: &Config) -> Result<()>
    where
        Command: Run,
    {
        let _config = config; // suppress unused-var warnings

        match &self.command {
            Some(command) => command.run(config)?,
            None => <Self as clap::CommandFactory>::command().print_help()?,
        }

        Ok(())
    }

    /// Returns the verbosity level of the log specified by the command line
    /// arguments.
    pub fn verbosity(&self) -> Option<Level> {
        self.verbosity.get()
    }

    /// Returns the subcommand specified by the command line arguments.
    pub fn command(&self) -> Option<&Command> {
        self.command.as_ref()
    }
}

/// Commmand line arguments to control log verbosity level.
///
/// # Examples
///
/// To get `--quiet` (`-q`) and `--verbose` (or `-v`) flags through your entire
/// program, just `flattern` this struct:
///
/// ```rust
/// use cli_xtask::{args::Verbosity, clap::Parser};
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     #[clap(flatten)]
///     verbosity: Verbosity,
/// }
/// ```
///
/// The [`LogLevel`](crate::tracing::Level) values returned by
/// [`Verbosity::get()`](crate::args::Verbosity::get) are:
///
/// * `None`: `-qqq`
/// * `Some(Level::ERROR)`: `-qq`
/// * `Some(Level::WARN)`: `-q`
/// * `Some(Level::INFO)`: no arguments
/// * `Some(Level::DEBUG)`: `-v`
/// * `Some(Level::TRACE)`: `-vv`
#[derive(Debug, clap::Args)]
pub struct Verbosity {
    /// More output per occurrence
    #[clap(long, short = 'v', parse(from_occurrences), global = true)]
    verbose: i8,
    /// Less output per occurrence
    #[clap(
        long,
        short = 'q',
        parse(from_occurrences),
        global = true,
        conflicts_with = "verbose"
    )]
    quiet: i8,
}

impl Verbosity {
    /// Returns the log verbosity level.
    pub fn get(&self) -> Option<Level> {
        let level = self.verbose - self.quiet;
        match level {
            i8::MIN..=-3 => None,
            -2 => Some(Level::ERROR),
            -1 => Some(Level::WARN),
            0 => Some(Level::INFO),
            1 => Some(Level::DEBUG),
            2..=i8::MAX => Some(Level::TRACE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbosity() {
        use clap::Parser;
        #[derive(Debug, Parser)]
        struct Args {
            #[clap(flatten)]
            verbosity: Verbosity,
        }
        impl Args {
            fn verbosity(&self) -> Option<Level> {
                self.verbosity.get()
            }
        }

        let cases: &[(&[&str], Option<Level>)] = &[
            (&["-qqqq"], None),
            (&["-qqq"], None),
            (&["-qq"], Some(Level::ERROR)),
            (&["-q"], Some(Level::WARN)),
            (&[], Some(Level::INFO)),
            (&["-v"], Some(Level::DEBUG)),
            (&["-vv"], Some(Level::TRACE)),
        ];

        for (arg, level) in cases {
            let args = Args::parse_from(["app"].into_iter().chain(arg.iter().copied()));
            assert_eq!(args.verbosity(), *level, "arg: {}", arg.join(" "));
        }
    }
}
