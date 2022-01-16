use crate::camera::Camera;
use crate::config::Settings;
use crate::hittable::hittable_list::HittableList;
use crate::ppm_file::image::PpmImage;
use crate::ppm_file::image_lazy::PpmImageLazy;
use crate::ray::Ray;
use crate::vector::Point3;
use crate::vector::Vec3;
use colored::Colorize;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod camera;
mod color_rgb;
mod config;
mod degrees;
mod hittable;
mod ppm_file;
mod ray;
mod sphere;
mod vector;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: Settings = Settings::new();
}

fn main() -> Result<(), Error> {
    const PATH: &str = "image.ppm";

    let mut world = HittableList::new();
    world.add(Vec3::new(0.0, 0.0, -1.0).with_radius(0.5));
    world.add(Vec3::new(0.0, -100.5, -1.0).with_radius(100.0));

    let width = SETTINGS.image_width;
    let height = SETTINGS.image_height;
    let samples_per_pixel = SETTINGS.samples_per_pixel;

    let camera = Camera::new();
    let bar = ProgressBar::new((width * height).into());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} | {elapsed_precise} | {bar:40.cyan/blue} | {pos:>7}/{len:7} | {msg}")
            .progress_chars("##-"),
    );
    let mut image_file = File::create(PATH)?;
    print!("\nGenerating Image\n\n");
    let image = PpmImageLazy::new(height, width).calculate(|row, column| {
        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        let v = 1.0 - (row as f64 + rng.gen::<f64>()) / (height - 1) as f64;
        let u = (column as f64 + rng.gen::<f64>()) / (width - 1) as f64;
        let ray = camera.get_ray(u, v);
        for _ in 1..=samples_per_pixel {
            pixel_color += ray.ray_color(&world, SETTINGS.max_depth)
        }
        let color = pixel_color.scale(samples_per_pixel).to_color_rgb_safe();
        bar.inc(1);
        bar.set_message(format!(
            "{}",
            "Processing".truecolor(color.red, color.green, color.blue)
        ));
        color
    });
    bar.finish_with_message(format!("{}", "Colors Generated!".green()));
    print!("\nSaving Image to ppm file:\n\n");
    write!(image_file, "{}", image)?;
    print!(
        "\n{}{}{}\n\n",
        "Image saved into ".green(),
        "Image.ppm".yellow(),
        " file. Enjoy!".green()
    );
    Ok(())
}
