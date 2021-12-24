use crate::sphere::hittable::Hittable;
use crate::sphere::HitRecord;
use crate::Ray;
use crate::Vec3;

pub(crate) struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hits_ray(self, r: &Ray) -> f32 {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0f32 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(self, _: &Ray, _: f32, _: f32, _: &HitRecord) -> bool {
        todo!()
    }
}
