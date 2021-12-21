use crate::Ray;
use crate::Vec3;

pub(crate) struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hits_ray(self, r: &Ray) -> f32 {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction) * 2f32;
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c) * 4f32;
        if discriminant < 0f32 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2f32 * a)
        }
    }
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}
