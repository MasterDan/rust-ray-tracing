use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::Ray;

pub(crate) struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn add<T: Hittable + 'static>(&mut self, item: T) {
        self.0.push(Box::new(item));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.0.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.update(&temp_rec);
            }
        }
        hit_anything
    }
}
