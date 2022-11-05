mod cli;
mod utils;


use clap::Parser;
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;

// use pqcrypto::kem::kyber1024::{keypair, encapsulate, decapsulate};


use cli::{Cli, Modes};
use utils::*;

fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    match &cli.mode {
        Modes::Encode(args) => { 
            encrypt_message(cli.password.to_string());
            println!("{:?}, {:?}", args.input_file, args.output_file);
        }
        Modes::Decode(_) => { todo!() }
    }
}