// Define my module
mod cli;
mod utils;

// My Modules
use cli::*;
use utils::*;

// External crates
use clap::Parser;
use image::io::Reader as ImageReader;
use image::ImageError;
use owo_colors::OwoColorize;
use std::path::Path;

fn main() -> Result<(), ImageError> {
    let args = StegoArgs::parse();

    match args.mode {
        Mode::Embed => {
            println!("Embedding...");

            let image = ImageReader::open(&args.image)?.decode()?;

            // Embed the message in the image
            let modified_image =
                embed_message_in_red_ch(image, args.message.expect("Message argument is required"));

            // Save the modified image to a file
            if let Some(output_path) = args.output {
                let path = Path::new(&output_path);
                // Save the output image to the specified path
                match modified_image.save(path) {
                    Ok(()) => println!("{} {:?}", "Image saved to:".green(), output_path),
                    Err(err) => println!(
                        "{} {} \nDid you specify the image extension? {}",
                        "Error:".red(),
                        err.red(),
                        "[ ~/path/to/image.png ]".green()
                    ),
                }
            } else {
                match modified_image.save("output.png") {
                    Ok(()) => println!("{}", "Image saved to output.png".green()),
                    Err(err) => println!("{}", err),
                }
            }
        }
        Mode::Extract => {
            println!("Extracting...");

            let image = image::open(args.image)?;

            if let Some(message) = extract_message_from_red_ch(image) {
                println!("Extracted message: {}", message);
            } else {
                println!("No message found in image");
            }
        }
    }

    Ok(())
}
