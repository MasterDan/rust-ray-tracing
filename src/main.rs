use crate::camera::Camera;
use crate::color_rgb::ColorRgb;
use crate::config::Settings;
use crate::hittable::hittable_list::HittableList;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
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
mod material;
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

    let world = HittableList::new_random();

    let width = SETTINGS.image_width;
    let height = SETTINGS.image_height;
    let samples_per_pixel = SETTINGS.samples_per_pixel;

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        SETTINGS.dist_to_focus,
    );
    let bar = ProgressBar::new((width * height).into());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} | {elapsed_precise} | {bar:50} | {msg}")
            .progress_chars("##-"),
    );
    print!("\nGenerating Image\n\n");
    let image = PpmImageLazy::new(height, width).calculate(|row, column| {
        let mut rng = rand::thread_rng();
        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let v = 1.0 - (row as f64 + rng.gen::<f64>()) / (height - 1) as f64;
            let u = (column as f64 + rng.gen::<f64>()) / (width - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color += ray.ray_color(&world, SETTINGS.max_depth)
        }
        let color = ColorRgb::from_vector_safe(pixel_color);
        bar.inc(1);
        let pos = bar.position() as f64;
        let len = bar.length() as f64;
        bar.set_message(format!(
            "{} | {:^6.2} %",
            "Processing".truecolor(color.red, color.green, color.blue),
            100.0 * pos / len
        ));
        color
    });
    bar.finish_with_message(format!("{}", "Colors Generated!".green()));
    print!("\nSaving Image to ppm file:\n\n");
    let mut image_file = File::create(PATH).expect("Error: Cannot create file!");
    write!(image_file, "{}", image)?;
    print!(
        "\n{}{}{}\n\n",
        "Image saved into ".green(),
        "Image.ppm".yellow(),
        " file. Enjoy!".green()
    );
    Ok(())
}
