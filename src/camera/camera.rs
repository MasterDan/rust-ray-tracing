use crate::degrees::Degrees;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use crate::SETTINGS;

pub(crate) struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, v_f_o_v: f64) -> Self {
        let theta = Degrees(v_f_o_v).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = SETTINGS.viewport_height * h;
        let viewport_width = SETTINGS.aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w)).unit();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
