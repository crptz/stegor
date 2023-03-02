use crate::cli::StegoArgs;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, Pixel, Rgba};
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

fn embed_message_in_red_ch(image: DynamicImage, message: String) -> DynamicImage {
    // Get the message bytes and length
    let message_bytes = message.into_bytes();
    let message_length = message_bytes.len();

    // Create a new image with the same dimensions as the original image
    let mut new_image = DynamicImage::new_rgba8(image.width(), image.height());

    // Embed the message length in the first pixel of the new image
    let first_pixel = Rgba([message_length as u8, 0, 0, 255]);
    new_image.put_pixel(0, 0, first_pixel);

    // Iterate over the pixels in the original image and embed the message bytes in the new image
    let mut byte_index = 0;
    for (x, y, pixel) in image.pixels() {
        // Skip the first pixel
        if x == 0 && y == 0 {
            continue;
        }

        // Get the original pixel color and embed the message byte in the red channel
        let mut new_pixel = pixel.to_rgba();
        if byte_index < message_length {
            new_pixel[0] = message_bytes[byte_index];
            byte_index += 1;
        }

        // Put the new pixel in the new image
        new_image.put_pixel(x, y, new_pixel);
    }

    new_image
}

fn extract_message_from_red_ch(image: DynamicImage) -> Option<String> {
    // Get the first pixel in the image
    let first_pixel = image.get_pixel(0, 0);

    // Extract the length of the message from the red channel of the first pixel
    let message_length = first_pixel[0] as usize;

    // Initialize a vector to hold the message bytes
    let mut message_bytes = vec![0u8; message_length];

    // Iterate over the pixels in the image containing the message bytes
    for (byte_index, (_, _, pixel)) in image.pixels().skip(1).take(message_length).enumerate() {
        let byte = pixel[0];
        message_bytes[byte_index] = byte;
    }

    /*
    For loop for accesing pixel positions, it may need in the future
    for (x, y, pixel) in image.pixels().skip(1).take(message_length) {

    }
     */

    // Convert the message bytes vector to a string
    String::from_utf8(message_bytes).ok()
}

// This function triggers whenever the user specifies the embed mode
pub fn embed_message(args: StegoArgs) -> Result<(), ImageError> {
    println!("Embedding...");

    let image_result = ImageReader::open(args.image.as_deref().unwrap_or_default())?.decode()?;
    let modified_image = embed_message_in_red_ch(
        image_result,
        args.message.expect("Message argument is required"),
    );

    let output_path = args.output.unwrap_or_else(|| {
        format!(
            "output.{}",
            args.image
                .as_deref()
                .and_then(|s| s.split('.').last())
                .unwrap_or("png")
        )
    });

    save_image(modified_image, output_path)
}

// This function triggers whenever the user specifies the extract mode
pub fn extract_message(args: StegoArgs) -> Result<(), ImageError> {
    println!("Extracting...");

    let image = ImageReader::open(args.image.as_deref().unwrap_or_default())?.decode()?;
    let message = extract_message_from_red_ch(image);

    match message {
        Some(message) => println!("Extracted message: {}", message),
        None => println!("No message found in image"),
    }

    Ok(())
}

// Print banner
pub fn print_banner(args: StegoArgs) {
    if args.is_empty() {
        println!(
            "{}{}{}",
            "Welcome to stegor!".purple(),
            ASCII_BANNER.cyan(),
            "v0.1.0".yellow()
        );
    }
}

// Save the image
pub fn save_image(image: DynamicImage, path: String) -> Result<(), ImageError> {
    match image.save(&path) {
        Ok(()) => println!("{} {:?}", "Image saved to:".green(), path),
        Err(err) => println!(
            "{} {} \nDid you specify the image extension? {}",
            "Error:".red(),
            err.red(),
            "[ ~/path/to/image.png ]".green()
        ),
    }

    Ok(())
}
