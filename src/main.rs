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
                if is_lossy_image(image_path.as_str()) {
                    println!("It's lossy")
                } else {
                    println!("It's lossless");
                    let image_result = match ImageReader::open(image_path) {
                        Ok(reader) => reader.decode(),
                        Err(e) => {
                            eprintln!("{} {}", "Error opening image:".red(), e);
                            return Ok(());
                        }
                    };
                
                    let image = image_result?;
    
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
