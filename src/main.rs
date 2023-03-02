// Define my module
mod cli;
mod utils;

// My Modules
use cli::*;
use utils::*;

// External crates
use clap::Parser;
use image::ImageError;

fn main() -> Result<(), ImageError> {
    let args = StegoArgs::parse();

    match args.mode {
        Some(Mode::Embed) => {
            embed_message(args).unwrap();
        }
        Some(Mode::Extract) => {
            extract_message(args).unwrap();
        }
        None => {
            print_banner(args);
        }
    }

    Ok(())
}
