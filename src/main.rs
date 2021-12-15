use crate::color_rgb::ColorRgb;
use crate::ppm_file::PpmImage;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod color_rgb;
mod ppm_file;

fn main() -> Result<(), Error> {
    let path = "image.ppm";
    let mut image_file = File::create(path)?;
    let width = 512;
    let height = 512;
    print!("Generating Image \n");
    let image = PpmImage::new(height, width, |row, column| {
        ColorRgb::new(
            ((column as f32 / (width as f32 - 1_f32)) * 255_f32) as u8,
            (((height as f32 - row as f32) / (height as f32 - 1_f32)) * 255_f32) as u8,
            (255.0 * 0.25) as u8,
        )
    });
    print!("Saving Image to ppm file\n");
    write!(image_file, "{}", image)?;
    Ok(())
}
