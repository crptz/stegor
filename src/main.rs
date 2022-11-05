mod cli;
mod utils;

// use std::path::PathBuf;


use clap::Parser;
use libaes::{AES_256_KEY_LEN, Cipher};
use passwords::PasswordGenerator;
use bytes::{BytesMut, BufMut};
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;
// use aes::Aes128;

use pqcrypto::kem::kyber1024::{keypair, encapsulate, decapsulate};
// use libaes::{Cipher, AES_256_KEY_LEN};
// use block_modes::block_padding::Pkcs7;
// use hex_literal::hex;
// use std::str;
// use std::env;


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
        Modes::Encode(args) => { 
            // println!("{:?} \n{:?}", files.input_file, files.output_file);

            encrypt_message(args.message.to_string());
            println!("{:?}, {:?}", args.input_file, args.output_file);
        }
        Modes::Decode(_) => { todo!() }
    }
}


fn encrypt_message(message: String) {
    
    println!("Before encryption: {:?}", message);
    
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

    // let mut buf = BytesMut::with_capacity(32);
    // buf.put_slice(&message.into().as_bytes()[..32]);
    // let x = "a";

    let message = b"dsajdiuashdiuashdiasdasdasdsadji";

    // let key:&[u8; AES_256_KEY_LEN] = k;
    let _cipher = Cipher::new_256(message);
}



// fn encode(input_file: PathBuf, output_file: PathBuf) {
//     
// }
// fn get_image(input_file: PathBuf) {
//     println!("{:?}", input_file);
// }

