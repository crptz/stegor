use image::io::Reader as ImageReader;
use image::DynamicImage;

pub fn load_image(image_path: &str) -> Result<DynamicImage, String> {
    match ImageReader::open(image_path) {
        Ok(reader) => Ok(reader.decode().map_err(|e| e.to_string())?),
        Err(e) => Err(format!("Error opening image: {}", e)),
    }
}
