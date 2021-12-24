use crate::Vec3;
use core::ops::Add;
use core::ops::Deref;
use core::ops::Sub;
#[derive(Clone, Copy, Debug)]
pub(crate) struct Point3(pub Vec3);

impl Deref for Point3 {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

impl Add<Vec3> for Point3 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        self.0 + vec
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        self.0 - vec
    }
}
