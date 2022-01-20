use crate::hittable::HitRecord;
use crate::ray::Ray;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
