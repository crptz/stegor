// Define my module
mod cli;
mod errors;
mod utils;

// My Modules
use cli::*;
use errors::*;
use utils::*;

// External crates
use clap::Parser;
use image::ImageError;
use owo_colors::OwoColorize;
const ASCII_BANNER: &'static str = r#"

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
            println!("Embedding...");

            // let open = &args.image.unwrap_or_default();

            if let Some(image_path) = args.image.as_ref() {

                let image = match load_image(image_path) {
                    Ok(image) => image,
                    Err(e) => {
                        eprintln!("{}", e);
                        return Ok(());
                    }
                };

                if is_lossy_image(image_path.as_str()) {
                    println!("It's lossy");
                    // Embed the message in the image
                    let modified_image = embed_message_in_blue_ch(
                        image,
                        args.message.expect("Message argument is required"),
                    );

                    save_image(image_path, args.output.as_deref(), &modified_image).unwrap();
                    
                } else {
                    println!("It's lossless");
                    // Embed the message in the image
                    let modified_image = embed_message_in_red_ch(
                        image,
                        args.message.expect("Message argument is required"),
                    );

                    save_image(image_path, args.output.as_deref(), &modified_image).unwrap();
                }
            } else {
                println!("No image provided");
            }
        }
        Some(Mode::Extract) => {
            println!("Extracting...");

            let image = image::open(args.image.unwrap_or_default())?;

            if let Some(message) = extract_message_from_red_ch(image) {
                println!("Extracted message: {}", message);
            } else {
                println!("No message found in image");
            }
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
