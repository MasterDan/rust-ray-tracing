use crate::material::Material;
use crate::Point3;
use crate::Vec3;
use std::sync::Arc;

pub(crate) struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material + 'a>,
}
