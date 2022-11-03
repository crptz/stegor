mod cli;

use clap::Parser;
use cli::{Cli, Modes};
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;

fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    // Since mode is Option, it returns either Some(mode) or None
    // so we make a match case for these two 
    match &cli.mode {
        Modes::Encode(_) => { todo!()}
        Modes::Decode(_) => { todo!() }
    }
}


// fn get_image(input_file: PathBuf) {
//     println!("{:?}", input_file);
// }

