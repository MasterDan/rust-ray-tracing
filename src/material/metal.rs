use crate::hittable::hit_record::HitRecord;
use crate::material::material::Attenuation;
use crate::material::material::Scattered;
use crate::material::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Copy)]
pub(crate) struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(x: f64, y: f64, z: f64) -> Metal {
        Metal {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord<'_>,
    ) -> std::option::Option<(Scattered, Attenuation)> {
        let reflected = Vec3::reflect(ray.direction.unit(), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        if Vec3::dot(scattered.direction, hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}
