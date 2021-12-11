use crate::color_rgb::ColorRgb;
use crate::ppm_file::PpmImage;

mod color_rgb;
mod ppm_file;

fn main() {
    let color = ColorRgb::new(255, 255, 0);
    let image = PpmImage::new(2, 2, |x, y| {
        ColorRgb::new(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            (x + y).try_into().unwrap(),
        )
    });
    println!(
        "Color is ({})\nColor in .ppm is {}",
        color,
        color.to_ppm_format()
    );
    println!("\n\nImage is:\n{}", image);
}
