use crate::color_rgb::color_rgb::ColorRgb;
use crate::sphere::sphere::Sphere;
use core::ops::Neg;
use core::ops::Sub;
use std::fmt::Formatter;
use std::fmt::{Display, Result};
use std::ops::AddAssign;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn to_color_rgb(self) -> ColorRgb {
        if [self.x, self.y, self.z]
            .iter()
            .any(|&x| x < 0f32 || x > 1f32)
        {
            println!("vec3 is {}", self);
            panic!("Only vectors between zero and one can be converted")
        }
        ColorRgb::new(
            (self.x * 255_f32) as u8,
            (self.y * 255_f32) as u8,
            (self.z * 255_f32) as u8,
        )
    }
    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }
    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }
    pub fn unit(self) -> Vec3 {
        self / 3.0
    }
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn make_sphere(self, radius: f32) -> Sphere {
        Sphere::new(self, radius)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, ext: Vec3) -> Vec3 {
        Vec3 {
            x: ext.x + self.x,
            y: ext.y + self.y,
            z: ext.z + self.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, ext: f32) -> Vec3 {
        Vec3 {
            x: ext + self.x,
            y: ext + self.y,
            z: ext + self.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, ext: f32) -> Self::Output {
        Vec3 {
            x: self.x * ext,
            y: self.y * ext,
            z: self.z * ext,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, ext: Vec3) -> Self::Output {
        Vec3 {
            x: ext.x * self,
            y: ext.y * self,
            z: ext.z * self,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        self * -1_f32
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, ext: f32) -> Self::Output {
        Vec3 {
            x: self.x / ext,
            y: self.y / ext,
            z: self.z / ext,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, minus: f32) -> Vec3 {
        Vec3 {
            x: self.x - minus,
            y: self.y - minus,
            z: self.z - minus,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec3;

    #[test]
    pub fn add() {
        let first = Vec3::new(1f32, 2f32, 3f32);
        let second = Vec3::new(4f32, 5f32, 6f32);
        let third = Vec3::new(5f32, 7f32, 9f32);
        assert_eq!(first + second, third);
    }

    #[test]
    pub fn mul() {
        let first = Vec3::new(1f32, 2f32, 3f32);
        let second = Vec3::new(2f32, 4f32, 6f32);
        assert_eq!(first * 2.0, second);
    }

    #[test]
    pub fn mul_assign() {
        let mut first = Vec3::new(1f32, 2f32, 3f32);
        first *= 2.0;
        let second = Vec3::new(2f32, 4f32, 6f32);
        assert_eq!(first, second);
    }

    #[test]
    pub fn div() {
        let first = Vec3::new(1f32, 2f32, 3f32);
        let second = Vec3::new(2f32, 4f32, 6f32);
        assert_eq!(second / 2.0, first);
    }

    #[test]
    pub fn div_assign() {
        let first = Vec3::new(1f32, 2f32, 3f32);
        let mut second = Vec3::new(2f32, 4f32, 6f32);
        second /= 2.0;
        assert_eq!(second, first);
    }
}
