use crate::color_rgb::color_rgb::ColorRgb;
use crate::hittable::hittable::Hittable;
use crate::vector::Point3;
use crate::Vec3;
use core::f64::INFINITY;

pub(crate) struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

type IsFrontFace = bool;
type Normal = Vec3;

impl Ray {
    pub fn get_face_normal(&self, outward_normal: Vec3) -> (IsFrontFace, Normal) {
        let front_face = self.direction.dot_with(outward_normal) < 0.0;
        if front_face {
            return (front_face, outward_normal);
        } else {
            return (front_face, -outward_normal);
        }
    }
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
    pub fn ray_color<T: Hittable>(self, world: &T) -> ColorRgb {
        if let Some(hit) = world.hit(&self, 0.0, INFINITY) {
            let vector = 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
            return vector.to_color_rgb();
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        let vector = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        vector.to_color_rgb()
    }
}
