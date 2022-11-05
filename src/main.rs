mod cli;
mod utils;


use clap::Parser;
use libaes::{ Cipher };
use passwords::PasswordGenerator;
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;

// use pqcrypto::kem::kyber1024::{keypair, encapsulate, decapsulate};


use cli::{Cli, Modes};


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


fn encrypt_message(password: String) {
    let _pg = PasswordGenerator {
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    // Hash the password with BLAKE3
    // basically the hashed_key is the password
    let hashed_key = blake3::hash(password.as_bytes());
    
    let plaintext = b"A plaintext";
    let iv = b"This is 16 bytes";

    // Create cipher instance from the hashed_key (password)
    let cipher = Cipher::new_256(hashed_key.as_bytes());

    let _encrypted = cipher.cbc_encrypt(iv, plaintext);

    println!("HASHED KEY: {:?}", hashed_key);
}

