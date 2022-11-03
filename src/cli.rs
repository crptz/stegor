use std::path::{PathBuf};
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "strs")]
#[command(author = "o3f")]
#[command(version = "1.0")]
#[command(about = "Adding data to images", long_about = None)]
/// Fields
/// mode: String < Has to options: 'en' or 'dec' >
/// input_file: PathBuf < The image we want to hide data to >
pub struct Cli {
    #[command(subcommand)]
    pub mode: Modes,    
    
    #[arg(short)]
    pub passowrd: String,
}


#[derive(Subcommand)]
pub enum Modes {
    Encode(File),
    Decode(Message)
}

#[derive(Args)]
pub struct File {
    #[arg(short)]
    pub input_file: PathBuf,

    #[arg(short)]
    pub output_file: PathBuf,

    #[arg(short)]
    pub message: String
}

#[derive(Args)]
pub struct Message {

}   