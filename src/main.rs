mod cli;
mod utils;

// use std::path::PathBuf;

use clap::Parser;
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
use std::env;


use cli::{Cli, Modes};
// use utils::*;


fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    // Since mode is Option, it returns either Some(mode) or None
    // so we make a match case for these two
     
    match &cli.mode {
        Modes::Encode(args) => { 
            // println!("{:?} \n{:?}", files.input_file, files.output_file);
            
            encrypt_message(args.message.to_string());
            println!("{:?}, {:?}", args.input_file, args.output_file);
        }
        Modes::Decode(_) => { todo!() }
    }
}


fn encrypt_message(message: String) {
    println!("{}", message)
}



// fn encode(input_file: PathBuf, output_file: PathBuf) {
//     
// }
// fn get_image(input_file: PathBuf) {
//     println!("{:?}", input_file);
// }

