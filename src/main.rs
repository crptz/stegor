// Define my module
mod cli;
mod utils;

// My Modules
use cli::*;
use utils::*;

// External crates
use clap::Parser;
use image::ImageError;
use owo_colors::OwoColorize;
const ASCII_BANNER: &str = r#"

     _                        
    | |                       
 ___| |_ ___  __ _  ___  _ __ 
/ __| __/ _ \/ _` |/ _ \| '__|
\__ \ ||  __/ (_| | (_) | |   
|___/\__\___|\__, |\___/|_|   
              __/ |           
             |___/            

"#;

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
            if args.is_empty() {
                println!(
                    "{}{}{}",
                    "Welcome to stegor!".purple(),
                    ASCII_BANNER.cyan(),
                    "v0.1.0".yellow()
                );
            }
        }
    }

    Ok(())
}
