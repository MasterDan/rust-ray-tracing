use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable::Hittable;
use crate::Ray;
use core::ops::Deref;
use core::ops::DerefMut;

pub(crate) struct HittableList<'a>(Vec<Box<dyn Hittable + 'a>>);

impl<'a> HittableList<'a> {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn add<T: Hittable + 'a>(&mut self, item: T) {
        self.push(Box::new(item));
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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

impl<'a> Deref for HittableList<'a> {
    type Target = Vec<Box<dyn Hittable + 'a>>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

impl DerefMut for HittableList<'_> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.0
    }
}
