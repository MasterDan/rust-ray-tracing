use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub(crate) struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0f32;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
