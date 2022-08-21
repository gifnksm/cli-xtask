use app::App;
use clap::Parser;

fn main() {
    let args = App::parse();
    println!("{args:?}");
}
