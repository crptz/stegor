use std::path::Path;
use libaes::{ Cipher };
use passwords::PasswordGenerator;
use image::{DynamicImage, open};

#[allow(dead_code)]
pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(Path::new(&filename)).unwrap();
    img
}

pub fn encrypt_message(password: String) {
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

    println!("HASHED KEY: {}", hashed_key.to_string());
}