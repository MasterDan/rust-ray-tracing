use crate::color_rgb::ColorRgb;
use crate::config::init_config;
use crate::ppm_file::PpmImage;
use crate::vector::Vec3;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod color_rgb;
mod config;
mod ppm_file;
mod ray;
mod vector;

fn main() -> Result<(), Error> {
    let settings = init_config();
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const PATH: &str = "image.ppm";
    let width = settings.get_int("image_width").unwrap() as u32;
    let height = (width as f32 / ASPECT_RATIO) as u32;
    let mut image_file = File::create(PATH)?;
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
