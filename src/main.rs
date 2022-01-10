use crate::config::Settings;
use crate::hittable::hittable_list::HittableList;
use crate::ppm_file::image::PpmImage;
use crate::ray::Ray;
use crate::vector::Point3;
use crate::vector::Vec3;
use std::fs::File;
use std::io::Error;
use std::io::Write;

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
    world.add(Vec3::new(0.0, 0.0, -1.0).make_sphere(0.5));
    world.add(Vec3::new(0.0, -100.5, -1.0).make_sphere(100.0));

    let width = SETTINGS.image_width;
    let height = SETTINGS.image_height;
    let viewport_height = SETTINGS.viewport_height;
    let focal_length = SETTINGS.focal_length;
    let viewport_width = SETTINGS.viewport_width;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut image_file = File::create(PATH)?;
    print!("\nGenerating Image");
    let image = PpmImage::new(height, width, |row, column| {
        let v = 1.0 - row as f64 / (height - 1) as f64;
        let u = column as f64 / (width - 1) as f64;
        let ray = Ray::new(
            origin,
            lower_left_corner + horizontal * u + vertical * v - origin,
        );
        ray.ray_color(&world)
    });
    print!("\nSaving Image to ppm file\n");
    write!(image_file, "{}", image)?;
    Ok(())
}
