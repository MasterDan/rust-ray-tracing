use crate::ColorRgb;
use crate::Vec3;

pub(crate) struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn ray_color(self) -> ColorRgb {
        let t = 0.5 * (self.direction.y + 1.0);
        let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        color.to_color_rgb()
    }
}
