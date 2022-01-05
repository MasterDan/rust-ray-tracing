use crate::color_rgb::color_rgb::ColorRgb;
use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::vector::Point3;
use crate::Vec3;
use core::f64::INFINITY;

pub(crate) struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        Point3(self.origin + t * self.direction)
    }
    pub fn ray_color<T: Hittable>(self, world: &T) -> ColorRgb {
        let hit_record = HitRecord::empty();
        if world.hit(&self, 0.0, INFINITY, &mut hit_record) {
            let vector = 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
            return vector.to_color_rgb();
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        let vector = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        vector.to_color_rgb()
    }
}
