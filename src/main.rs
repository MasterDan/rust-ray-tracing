use crate::config::config::init_config;
use crate::hittable::hittable_list::HittableList;
use crate::ppm_file::image::PpmImage;
use crate::ray::ray::Ray;
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

fn main() -> Result<(), Error> {
    let settings = init_config();
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const PATH: &str = "image.ppm";

    let mut world = HittableList::new();
    world.add(Vec3::new(0.0, 0.0, -1.0).make_sphere(0.5));
    world.add(Vec3::new(0.0, -100.5, -1.0).make_sphere(100.0));

    let width = settings.get_int("image_width").unwrap() as u32;
    let viewport_height = settings.get_float("viewport_height").unwrap();
    let focal_length = settings.get_float("focal_length").unwrap();
    let viewport_width = ASPECT_RATIO * viewport_height;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let height = (width as f64 / ASPECT_RATIO) as u32;
    let mut image_file = File::create(PATH)?;
    print!("\nGenerating Image");
    let image = PpmImage::new(height, width, |row, column| {
        let v = 1.0 - row as f64 / (height - 1) as f64;
        let u = column as f64 / (width - 1) as f64;
        let ray = Ray::new(
            Point3(origin),
            lower_left_corner + horizontal * u + vertical * v - origin,
        );
        ray.ray_color(&world)
    });
    print!("\nSaving Image to ppm file\n");
    write!(image_file, "{}", image)?;
    Ok(())
}
