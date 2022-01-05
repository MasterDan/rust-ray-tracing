use core::ops::Add;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use std::fmt::{Display, Error, Formatter};
use std::result::Result;

use super::color_rgb_ppm::ColorRgbPpm;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColorRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRgb {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn to_ppm_format(self) -> ColorRgbPpm {
        ColorRgbPpm(self)
    }
}

impl Display for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl Add for ColorRgb {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ColorRgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul<f64> for ColorRgb {
    type Output = Self;
    fn mul(self, mult: f64) -> Self {
        ColorRgb {
            red: (self.red as f64 * mult) as u8,
            green: (self.green as f64 * mult) as u8,
            blue: (self.blue as f64 * mult) as u8,
        }
    }
}

impl MulAssign<f64> for ColorRgb {
    fn mul_assign(&mut self, mul: f64) {
        self.red = (self.red as f64 * mul) as u8;
        self.green = (self.green as f64 * mul) as u8;
        self.blue = (self.blue as f64 * mul) as u8;
    }
}

impl Div<f64> for ColorRgb {
    type Output = Self;
    fn div(self, div: f64) -> Self {
        if div == 0.0 {
            panic!("Divide by Zero")
        }
        ColorRgb {
            red: (self.red as f64 / div) as u8,
            green: (self.green as f64 / div) as u8,
            blue: (self.blue as f64 / div) as u8,
        }
    }
}

impl DivAssign<f64> for ColorRgb {
    fn div_assign(&mut self, div: f64) {
        self.red = (self.red as f64 / div) as u8;
        self.green = (self.green as f64 / div) as u8;
        self.blue = (self.blue as f64 / div) as u8;
    }
}

#[cfg(test)]
mod tests {
    use crate::color_rgb::color_rgb::ColorRgb;

    #[test]
    pub fn add() {
        let first = ColorRgb::new(1, 2, 3);
        let second = ColorRgb::new(4, 5, 6);
        let third = ColorRgb::new(5, 7, 9);
        assert_eq!(first + second, third);
    }
    #[test]
    pub fn mul() {
        let first = ColorRgb::new(1, 2, 3);
        let second = ColorRgb::new(2, 4, 6);
        assert_eq!(first * 2.0, second);
    }

    #[test]
    pub fn mul_assign() {
        let mut first = ColorRgb::new(1, 2, 3);
        first *= 2.0;
        let second = ColorRgb::new(2, 4, 6);
        assert_eq!(first, second);
    }

    #[test]
    pub fn div() {
        let first = ColorRgb::new(1, 2, 3);
        let second = ColorRgb::new(2, 4, 6);
        assert_eq!(second / 2.0, first);
    }

    #[test]
    pub fn div_assign() {
        let first = ColorRgb::new(1, 2, 3);
        let mut second = ColorRgb::new(2, 4, 6);
        second /= 2.0;
        assert_eq!(second, first);
    }
}
