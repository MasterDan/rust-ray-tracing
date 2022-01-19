use crate::hittable::hit_record::HitRecord;
use crate::Ray;
use crate::Vec3;

pub(crate) trait Material {
    fn scatter(ray: &Ray, hit: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
}
