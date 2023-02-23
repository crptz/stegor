use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author="pwn_it@unavoidable0100", version="0.1.0", about="A steganography tool too extract and hide data inside images and more", long_about = None)]
#[command(propagate_version = true)]
pub struct StegoArgs {
    /// The mode of operation for the steganography program.
    #[arg(value_enum)]
    pub mode: Mode,
    /// The input image for the steganography program.
    #[clap(short, long, required = true)]
    pub image: String,
    /// The message to be hidden in the input image.
    #[clap(short, long)]
    pub message: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Embed,
    Extract,
}