use crate::hittable::hit_record::HitRecord;
use crate::material::material::Attenuation;
use crate::material::material::Scattered;
use crate::material::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Copy)]
pub(crate) struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fz < 1.0 { 1.0 } else { fz },
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
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if Vec3::dot(scattered.direction, hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}
