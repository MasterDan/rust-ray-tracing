use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub(crate) struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

type IsFrontFace = bool;
type Normal = Vec3;

impl HitRecord {
    pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (IsFrontFace, Normal) {
        let front_face = ray.direction.dot_with(outward_normal) < 0.0;
        if front_face {
            return (front_face, outward_normal);
        } else {
            return (front_face, -outward_normal);
        }
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
