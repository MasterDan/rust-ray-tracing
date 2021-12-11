use crate::color_rgb::ColorRgb;
use crate::ppm_file::PpmImage;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod color_rgb;
mod ppm_file;

fn main() -> Result<(), Error> {
    let path = "image.ppm";
    let mut output = File::create(path)?;
    let image = PpmImage::new(256, 256, |x, y| {
        ColorRgb::new(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            (255 - y).try_into().unwrap(),
        )
    });
    write!(output, "{}", image)?;
    Ok(())
}
