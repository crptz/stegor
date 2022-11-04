use std::path::Path;

use image::{DynamicImage, open};

#[allow(dead_code)]
pub fn str_to_bytes<'a>(msg: &'a String) -> &'a [u8] {
    msg.as_bytes()
}

#[allow(dead_code)]
pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(&Path::new(&filename)).unwrap();
    img
}