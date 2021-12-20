use crate::Vec3;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Sub;

#[derive(Clone, Copy)]
pub(crate) struct Dot {
    pub x: Vec3,
    pub y: Vec3,
}

impl Dot {
    pub fn new(x: Vec3, y: Vec3) -> Dot {
        Dot { x, y }
    }
}

impl Add for Dot {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Dot {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<f32> for Dot {
    type Output = Dot;
    fn sub(self, minus: f32) -> Self {
        Dot {
            x: self.x - minus,
            y: self.y - minus,
        }
    }
}

impl Sub for Dot {
    type Output = Dot;
    fn sub(self, other: Dot) -> Self {
        Dot {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Dot {
    type Output = Dot;

    fn mul(self, other: Dot) -> Self {
        Dot {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Dot {
    type Output = Self;

    fn mul(self, mul: f32) -> Self {
        Dot {
            x: self.x * mul,
            y: self.y * mul,
        }
    }
}
