use std::path::Path;
use libaes::{ Cipher };
use passwords::PasswordGenerator;
use image::{DynamicImage, open};

#[allow(dead_code)]
pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(Path::new(&filename)).unwrap();
    img
}

fn pass_gen() -> Result<std::string::String, &'static str> {
    let pg = PasswordGenerator {
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };
    pg.generate_one()
}

pub fn encrypt_message(password: String) {

    // Hash the password with BLAKE3
    // basically the hashed_key is the password
    let hashed_key = blake3::hash(password.as_bytes());
    
    let plaintext = b"Hi";


    // Ensure that the Return Type is String
    match pass_gen() {
        Ok(iv) => { 
            println!("{}", iv);
            
                // Create cipher instance from the hashed_key (password)
            let cipher = Cipher::new_256(hashed_key.as_bytes());

            let encrypted = cipher.cbc_encrypt(iv.as_bytes(), plaintext).to_vec();
            

            // Convert Vec<u8> to String //
            // let en_text = String::from_utf8(encrypted).unwrap();

            // return &(iv + &en_text.to_string());
        },
        Err(err) => { println!("Generated password is not String. \n Error message: {}", err) }
    }

    println!("YOUR PASSWORD: {}", hashed_key.to_string());
}


fn write_to_file() {
    // The basic idea is to write the encrypted message as String to a file
    // so we can then read the data from the file and decrypted
}

#[allow(dead_code)]
pub fn decrypt_message() {
    /* 
    let cipher = Cipher::new_256(hashed_key.as_bytes());

    let decrypted = cipher.cbc_decrypt(iv.as_bytes(), &encrypted).to_vec();

    let str2 = String::from_utf8(decrypted).unwrap();
    
    println!("DECRYPTED MESSAGE: {}", str2);
    */
}