use crate::dot::Dot;
use crate::Ray;
use crate::Vec3;

pub(crate) struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hits_ray(self, r: Ray) -> bool {
        let oc = r.origin - self.center;
        let a = Dot::new(r.direction, r.direction);
        let b = Dot::new(oc, r.direction) * 2.0;
        let c = Dot::new(oc, oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c) * 4.0;
        // return (discriminant > 0);
        todo!()
    }
}
