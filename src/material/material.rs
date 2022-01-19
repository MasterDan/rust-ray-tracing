use crate::hittable::hit_record::HitRecord;
use crate::Ray;
use crate::Vec3;

pub(crate) type Scattered = Ray;
pub(crate) type Attenuation = Vec3;

pub(crate) trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Scattered, Attenuation)>;
}
