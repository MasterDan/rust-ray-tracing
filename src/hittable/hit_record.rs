use crate::Point3;
use crate::Vec3;

pub(crate) struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
