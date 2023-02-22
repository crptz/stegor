use clap::{Parser, ValueEnum};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, Pixel, Rgba};

#[derive(Parser)]
#[command(author="pwn_it@unavoidable0100", version="0.1.0", about="A steganography tool too extract and hide data inside images and more", long_about = None)]
#[command(propagate_version = true)]
struct StegoArgs {
    /// The mode of operation for the steganography program.
    /// Allowed values: "embed" or "extract".
    #[arg(value_enum)]
    mode: Mode,
    /// The input file for the steganography program.
    #[clap(short, long, required = true)]
    file: String,
    /// The message to be hidden in the input file.
    #[clap(short, long)]
    message: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Embed,
    Extract,
}

fn main() -> Result<(), ImageError> {
    let args = StegoArgs::parse();

    match args.mode {
        Mode::Embed => {
            println!("Embedding...");

            let image = ImageReader::open(&args.file)?.decode()?;

            // Embed the message in the image
            let modified_image =
                embed_message_in_image(image, args.message.expect("Message argument is required"));

            // Save the modified image to a file
            modified_image.save("output.png")?;
        }
        Mode::Extract => {
            println!("Extracting...");

            let image = image::open(args.file)?;

            if let Some(message) = extract_message_from_image(image) {
                println!("Extracted message: {}", message);
            } else {
                println!("No message found in image");
            }
        }
    }

    Ok(())
}

fn embed_message_in_image(image: DynamicImage, message: String) -> DynamicImage {
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

fn extract_message_from_image(image: DynamicImage) -> Option<String> {
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
