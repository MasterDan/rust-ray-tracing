use crate::hittable::HitRecord;
use crate::material::material::Attenuation;
use crate::material::material::Scattered;
use crate::material::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Copy)]
pub(crate) struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(x: f64, y: f64, z: f64) -> Lambertian {
        Lambertian {
            albedo: Vec3::new(x, y, z),
        }
    }

    pub fn from_vector(v: Vec3) -> Lambertian {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord<'_>) -> Option<(Scattered, Attenuation)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal
        }
        let scattered_ray = Ray::new(rec.p, scatter_dir);
        Some((scattered_ray, self.albedo))
    }
}
