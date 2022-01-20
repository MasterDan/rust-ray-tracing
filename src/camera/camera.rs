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
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3) -> Self {
        let theta = Degrees(SETTINGS.vfow).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = SETTINGS.viewport_height * h;
        let viewport_width = SETTINGS.aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w)).unit();
        let v = Vec3::cross(w, u);

        let focus_dist = (lookfrom - lookat).length();
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            v,
            w,
            lens_radius: SETTINGS.aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
