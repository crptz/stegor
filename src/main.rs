mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // For debugging
    println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());


    match &cli.mode {
        Some(mode) => {
            match mode.as_str() {
                "en" => println!("ENCODING"),
                "dec" => println!("ENCODING"),
                _ => println!("Consider using one of the modes. use --help for more info")
            }
        },
        None => println!("Consider using one of the modes. use --help for more info"),
    }
}
