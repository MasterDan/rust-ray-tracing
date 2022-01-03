use crate::hittable::hit_record::HitRecord;
use crate::Ray;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool;
}
