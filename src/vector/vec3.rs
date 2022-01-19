use crate::color_rgb::ColorRgb;
use crate::sphere::Sphere;
use crate::SETTINGS;
use core::ops::Neg;
use core::ops::Sub;
use rand::Rng;
use std::fmt::Formatter;
use std::fmt::{Display, Result};
use std::ops::AddAssign;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub(crate) type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:>7.3}, {:>7.3}, {:>7.3})", self.x, self.y, self.z)
    }
}

fn clamp_float(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn to_color_rgb(self) -> ColorRgb {
        if [self.x, self.y, self.z]
            .iter()
            .any(|&x| x < 0f64 || x > 1f64)
        {
            println!("\nvec3 is {}", self);
            panic!("Only vectors between zero and one can be converted")
        }
        ColorRgb::new(
            (self.x * 255_f64) as u8,
            (self.y * 255_f64) as u8,
            (self.z * 255_f64) as u8,
        )
    }

    fn to_color_rgb_dirty(self) -> ColorRgb {
        ColorRgb {
            red: self.x as u8,
            green: self.y as u8,
            blue: self.z as u8,
        }
    }

    pub fn map_values<T: Fn(f64) -> f64>(self, map: T) -> Vec3 {
        Vec3::new(map(self.x), map(self.y), map(self.z))
    }

    pub fn to_color_rgb_safe(self) -> ColorRgb {
        let scale = 1.0 / SETTINGS.samples_per_pixel as f64;
        let result = self.map_values(|x| {
            let scaled = (x * scale).sqrt();
            let result = 255.0 * clamp_float(scaled, 0.0, 1.0);
            result.round()
        });
        result.to_color_rgb_dirty()
    }

    pub fn dot_with(self, other: Vec3) -> f64 {
        Vec3::dot(self, other)
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
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
        self / self.length()
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    pub fn random_interval(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_interval(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn with_radius(self, radius: f64) -> Sphere {
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

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, ext: Vec3) -> Vec3 {
        Vec3 {
            x: ext.x + self,
            y: ext.y + self,
            z: ext.z + self,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, ext: f64) -> Vec3 {
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

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, ext: f64) -> Self::Output {
        Vec3 {
            x: self.x * ext,
            y: self.y * ext,
            z: self.z * ext,
        }
    }
}

impl Mul<Vec3> for f64 {
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
        self * -1_f64
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, ext: f64) -> Self::Output {
        Vec3 {
            x: self.x / ext,
            y: self.y / ext,
            z: self.z / ext,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
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

impl Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self - vec.x,
            y: self - vec.y,
            z: self - vec.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, minus: f64) -> Vec3 {
        Vec3 {
            x: self.x - minus,
            y: self.y - minus,
            z: self.z - minus,
        }
    }
}
