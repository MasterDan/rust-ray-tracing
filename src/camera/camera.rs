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
    pub fn new(v_f_o_v: f64) -> Self {
        let theta = Degrees(v_f_o_v).toRadians();
        let h = (theta / 2.0).tan();
        let viewport_height = SETTINGS.viewport_height * h;
        let viewport_width = SETTINGS.aspect_ratio * viewport_height;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, SETTINGS.focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
