use std::path::{PathBuf};
use clap::{Parser};

#[derive(Parser)]
#[command(name = "strs")]
#[command(author = "o3f")]
#[command(version = "1.0")]
#[command(about = "Adding data to images", long_about = None)]
pub struct Cli {

    pub mode: Option<String>,    
    
    #[arg(short)]
    pub input_file: PathBuf,    
}