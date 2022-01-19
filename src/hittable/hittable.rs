use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
