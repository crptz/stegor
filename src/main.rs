use clap::Parser;
use hips_lib::hips::{find_secret_img, hide_secret_img};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct StegoArgs {
    /// The mode of operation for the steganography program.
    /// Allowed values: "embed" or "extract".
    #[arg(value_enum)]
    mode: String,
    /// The input file for the steganography program.
    #[clap(short, long, required = true)]
    file: String,
    /// The message to be hidden in the input file.
    #[clap(short, long)]
    message: Option<String>,
}

fn main() {
    let args = StegoArgs::parse();

    match args.mode.as_str() {
        "embed" => {
            println!("Encoding");

            // image::open(&args.file).expect("Failed to open image");

            // args.message.expect("Message argument is required");
            let secret = String::from("hiddenimage");
            let result_img = if let Some(_message) = args.message {
                hide_secret_img(&args.file, &secret)
            } else {
                panic!();
            };

            assert!(result_img.is_ok());
        }
        "extract" => {
            println!("Decoding");

            let result = find_secret_img(&args.file).unwrap();
            // assert!(result.is_some());
            match result {
                Some(value) => {
                    println!("{}", value);
                }
                None => println!("No value is present"),
            }
        }
        _ => {
            println!("Invalid Mode.")
        }
    }

    // println!("mode: {0} , file: {1} , message: {2:?}", args.mode, args.file, args.message);
}
