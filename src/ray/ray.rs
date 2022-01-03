use crate::color_rgb::color_rgb::ColorRgb;
use crate::vector::Point3;
use crate::Vec3;

pub(crate) struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f32) -> Point3 {
        Point3(self.origin + t * self.direction)
    }
    pub fn ray_color(self) -> ColorRgb {
        let t = Vec3::new(0f32, 0f32, -1f32)
            .make_sphere(0.5)
            .hits_ray(&self);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0f32, 0f32, -1f32)).unit();
            return (0.5 * (n + 1.0)).to_color_rgb();
        }
        let t = 0.5 * (self.direction.unit().y + 1.0);
        let color = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        color.to_color_rgb()
    }
}
