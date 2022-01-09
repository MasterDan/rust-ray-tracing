use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::Ray;
use crate::Vec3;

pub(crate) struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hits_ray(self, r: &Ray) -> f64 {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0f64 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
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
        let (ff, norm) = HitRecord::set_face_normal(ray, outward_normal);
        let hit = HitRecord {
            t: root,
            p: temp_point,
            front_face: ff,
            normal: norm,
        };
        Some(hit)
    }
}
