mod cli;

use cli::CLI;
use clap::Parser;

fn main() {
    let cli = CLI::parse();
    println!("{:?}", cli);
}
