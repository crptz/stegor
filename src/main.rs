mod cli;
mod utils;

use std::path::PathBuf;

use clap::Parser;
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;


use cli::{Cli, Modes};
use utils::*;


fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    // Since mode is Option, it returns either Some(mode) or None
    // so we make a match case for these two
     
    match &cli.mode {
        Modes::Encode(files) => { 
            // println!("{:?} \n{:?}", files.input_file, files.output_file);
            
            // encode(files.input_file, files.output_file);
        }
        Modes::Decode(_) => { todo!() }
    }
}

fn encode(input_file: PathBuf, output_file: PathBuf) {
    
}
// fn get_image(input_file: PathBuf) {
//     println!("{:?}", input_file);
// }

