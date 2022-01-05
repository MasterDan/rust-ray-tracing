use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub(crate) struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0f64;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn empty() -> Self {
        HitRecord {
            p: Point3(Vec3::new(0.0, 0.0, 0.0)),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn update(&mut self, other: &HitRecord) {
        self.front_face = other.front_face;
        self.p = other.p;
        self.t = other.t;
        self.normal = other.normal;
    }
}
