use crate::camera::Camera;
use crate::config::Settings;
use crate::hittable::hittable_list::HittableList;
use crate::ppm_file::image::PpmImage;
use crate::ray::Ray;
use crate::vector::Point3;
use crate::vector::Vec3;
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

    let mut image_file = File::create(PATH)?;
    print!("\nGenerating Image");
    let image = PpmImage::new(height, width, |row, column| {
        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        for _ in 1..=samples_per_pixel {
            let v = 1.0 - (row as f64 + rng.gen::<f64>()) / (height - 1) as f64;
            let u = (column as f64 + rng.gen::<f64>()) / (width - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color += ray.ray_color(&world)
        }
        pixel_color.scale(samples_per_pixel).to_color_rgb_safe()
    });
    print!("\nSaving Image to ppm file\n");
    write!(image_file, "{}", image)?;
    Ok(())
}
