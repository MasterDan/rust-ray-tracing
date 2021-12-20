use crate::ColorRgb;
use std::fmt::Formatter;
use std::fmt::{Display, Result};
use std::ops::AddAssign;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
        ColorRgb {
            red: (self.x * 255_f32) as u8,
            green: (self.y * 255_f32) as u8,
            blue: (self.z * 255_f32) as u8,
        }
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

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
