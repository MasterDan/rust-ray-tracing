use crate::Ray;
use crate::Vec3;

pub(crate) struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hits_ray(self, r: Ray) -> bool {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction) * 2f32;
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c) * 4f32;
        return discriminant > 0f32;
    }
}
