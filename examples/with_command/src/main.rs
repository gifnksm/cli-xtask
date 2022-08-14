use clap::Parser;
use with_command::Args;

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
