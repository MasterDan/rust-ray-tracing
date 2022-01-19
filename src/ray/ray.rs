use crate::hittable::Hittable;
use crate::vector::Point3;
use crate::Vec3;

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
    pub fn ray_color<T: Hittable>(&self, world: &T, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(self, 0.0, f64::MAX) {
            let target: Point3 = hit.p + hit.normal + Vec3::random_in_unit_sphere();
            return 0.5 * Ray::new(hit.p, target - hit.p).ray_color(world, depth - 1);
        }

        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
