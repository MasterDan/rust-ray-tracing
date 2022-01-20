use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;
use std::sync::Arc;

pub(crate) struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new<T: Material + 'a>(center: Vec3, radius: f64, material: T) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            mat_ptr: Arc::new(material),
        }
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0_f64 {
            return None;
        };
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let temp_point = ray.at(root);
        let outward_normal: Vec3 = (temp_point - self.center) / self.radius;
        let (ff, norm) = ray.get_face_normal(outward_normal);
        let hit = HitRecord {
            t: root,
            p: temp_point,
            front_face: ff,
            normal: norm,
            mat_ptr: self.mat_ptr.clone(),
        };
        Some(hit)
    }
}
