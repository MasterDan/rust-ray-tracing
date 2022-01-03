use crate::hittable::hittable::Hittable;

pub(crate) struct HittableList(Vec<Box<dyn Hittable>>);
