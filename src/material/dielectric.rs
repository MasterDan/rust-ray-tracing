use crate::hittable::HitRecord;
use crate::material::Material;
use crate::Ray;
use crate::Vec3;
use rand::Rng;
#[derive(Clone, Copy)]
pub(crate) struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord<'_>) -> std::option::Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();
        let cos_theta = Vec3::dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
            Vec3::reflect(unit_direction, hit.normal)
        } else {
            Vec3::refract(unit_direction, hit.normal, refraction_ratio)
        };
        let scattered = Ray::new(hit.p, direction);
        return Some((scattered, attenuation));
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1_f64 - ref_idx) / (1_f64 + ref_idx)).powi(2);
    return r0 + (1_f64 - r0) * (1_f64 - cosine).powi(5);
}
