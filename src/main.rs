mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    match &cli.mode {
        Some(mode) => {
            println!("{}", mode)
        },
        None => println!("Consider using a specific mode"),
    }
}
