//! Example CLI application.

use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Args {
    /// The path to the file to read.
    #[clap(long)]
    pub input: PathBuf,
    /// The path to the file to write.
    #[clap(long)]
    pub output: PathBuf,
    #[clap(subcommand)]
    pub command: Subcommand,
}

#[derive(Debug, clap::Parser)]
pub enum Subcommand {
    /// Copy the input file to the output file.
    Copy {
        /// Force the copy to overwrite an existing output file.
        #[clap(short, long)]
        force: bool,
        #[clap(subcommand)]
        command: CopyCommand,
    },
    /// Move the contents of the input file.
    Move {
        /// Force the move to overwrite an existing output file.
        #[clap(short, long)]
        force: bool,
        #[clap(subcommand)]
        command: MoveCommand,
    },
}

#[derive(Debug, clap::Parser)]
pub enum CopyCommand {
    /// Execute the copy command.
    Execute,
    /// Print the copy command.
    DryRun,
}

#[derive(Debug, clap::Parser)]
pub enum MoveCommand {
    /// Execute the move command.
    Execute,
    /// Print the move command.
    DryRun,
}
