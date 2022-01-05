use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::Ray;
use core::ops::Deref;
use core::ops::DerefMut;

pub(crate) struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn add<T: Hittable + 'static>(&mut self, item: T) {
        self.push(Box::new(item));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.update(&temp_rec);
            }
        }
        hit_anything
    }
}

impl Deref for HittableList {
    type Target = Vec<Box<dyn Hittable>>;
    fn deref(&self) -> &Vec<Box<dyn Hittable>> {
        &self.0
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Vec<Box<dyn Hittable>> {
        &mut self.0
    }
}
