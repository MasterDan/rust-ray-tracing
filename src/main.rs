use crate::color_rgb::ColorRgb;
use crate::ppm_file::PpmImage;
use crate::vector::Vec3;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod color_rgb;
mod ppm_file;
mod vector;

fn main() -> Result<(), Error> {
    let path = "image.ppm";
    let mut image_file = File::create(path)?;
    let width = 256;
    let height = 256;
    print!("Generating Image \n");
    let image = PpmImage::new(height, width, |row, column| {
        Vec3::new(
            column as f32 / (width - 1) as f32,
            1.0 - (row as f32 / (height - 1) as f32),
            0.25,
        )
        .to_color_rgb()
    });
    print!("Saving Image to ppm file\n");
    write!(image_file, "{}", image)?;
    Ok(())
}
