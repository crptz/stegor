use std::{error::Error, path::Path};

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use owo_colors::OwoColorize;

pub fn embed_message_in_red_ch(image: DynamicImage, message: String) -> DynamicImage {
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

pub fn extract_message_from_red_ch(image: DynamicImage) -> Option<String> {
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

pub fn is_lossy_image(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    match ext {
        "jpg" | "jpeg" | "webp" | "heif" => true,
        "png" | "gif" | "bmp" => false,
        _ => panic!("Unknown file extension"),
    }
}

pub fn save_image(
    input_path: &str,
    output_path: Option<&str>,
    modified_image: &DynamicImage,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&input_path);
    let mut output_file_path = String::new();

    // If output path is provided, use that, else use default output file name
    if let Some(path) = output_path {
        output_file_path = path.to_string();
    } else {
        let input_file_name = path
            .file_stem()
            .ok_or("Invalid input file path")?
            .to_str()
            .ok_or("Invalid input file path")?;
        output_file_path = format!("{}_output.png", input_file_name);
    }

    // Save the modified image to a file
    let output_path = Path::new(&output_file_path);
    match modified_image.save(output_path) {
        Ok(()) => println!("{} {:?}", "Image saved to:".green(), output_path),
        Err(err) => println!(
            "{} {} \nDid you specify the image extension? {}",
            "Error:".red(),
            err.red(),
            "[ ~/path/to/image.png ]".green()
        ),
    }

    Ok(())
}

/*
use image::{DynamicImage, Rgba};

// Embed a message in an image using spread spectrum steganography
pub fn embed_message_with_spread_spectrum(image: DynamicImage, message: &str, key: &str) -> DynamicImage {
    // Get the message bytes and length
    let message_bytes = message.as_bytes();
    let message_length = message_bytes.len();

    // Get the key bytes and length
    let key_bytes = key.as_bytes();
    let key_length = key_bytes.len();

    // Create a new image with the same dimensions as the original image
    let mut new_image = DynamicImage::new_rgba8(image.width(), image.height());

    // Embed the message length in the first pixel of the new image
    let first_pixel = Rgba([message_length as u8, key_length as u8, 0, 255]);
    new_image.put_pixel(0, 0, first_pixel);

    // Calculate the number of color channels to use for embedding the message
    let num_channels = (message_length / key_length) + 1;

    // Embed the message bytes in the image by spreading them across multiple color channels
    let mut channel_index = 0;
    for (x, y, pixel) in image.pixels() {
        // Skip the first pixel
        if x == 0 && y == 0 {
            continue;
        }

        // Get the original pixel color and embed the message byte in the current channel
        let mut new_pixel = pixel.to_rgba();
        if channel_index < num_channels && channel_index % key_length == 0 {
            let key_byte = key_bytes[channel_index % key_length];
            let message_byte = message_bytes[channel_index / key_length];
            let encrypted_byte = message_byte ^ key_byte;
            new_pixel[channel_index % 3] = encrypted_byte;
            channel_index += 1;
        }

        // Put the new pixel in the new image
        new_image.put_pixel(x, y, new_pixel);
    }

    new_image
}

 */
