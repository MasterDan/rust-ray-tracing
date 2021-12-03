use crate::color_rgb::ColorRgb;

mod color_rgb;

fn main() {
    let color = ColorRgb::new(255, 255, 0);
    println!(
        "Color is ({})\nColor in .ppm is {}",
        color,
        color.to_ppm_format()
    );
}
