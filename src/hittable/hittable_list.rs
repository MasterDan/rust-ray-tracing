use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::Dielectric;
use crate::Lambertian;
use crate::Metal;
use crate::Point3;
use crate::Vec3;
use core::ops::Deref;
use core::ops::DerefMut;
use rand::thread_rng;
use rand::Rng;

pub(crate) struct HittableList<'a>(Vec<Box<dyn Hittable + Send + Sync + 'a>>);

impl<'a> HittableList<'a> {
    pub fn _clear(&mut self) {
        self.0.clear();
    }

    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn add<T: Hittable + Send + Sync + 'a>(&mut self, item: T) {
        self.push(Box::new(item));
    }

    pub fn new_random() -> Self {
        let mut world = HittableList::new();

        let ground_material = Lambertian::new(0.5, 0.5, 0.5);
        world.add(Point3::new(0.0, -1000.0, 0.0).make_sphere(1000.0, ground_material));

        let mut rng = thread_rng();
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f64 = rng.gen();
                let center = Point3::new(
                    (a as f64) + 0.9 * rng.gen::<f64>(),
                    0.2,
                    (b as f64) + 0.9 * rng.gen::<f64>(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random() * Vec3::random();
                        let sphere_material = Lambertian::from_vector(albedo);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random_interval(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    } else {
                        // glass
                        let sphere_material = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

        let material2 = Lambertian::new(0.4, 0.2, 0.1);
        world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

        let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
        world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

        return world;
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}

impl<'a> Deref for HittableList<'a> {
    type Target = Vec<Box<dyn Hittable + Send + Sync + 'a>>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

impl DerefMut for HittableList<'_> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.0
    }
}
