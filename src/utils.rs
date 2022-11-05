use std::path::Path;

use image::{DynamicImage, open};

#[allow(dead_code)]
pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(Path::new(&filename)).unwrap();
    img
}