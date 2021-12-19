use std::fmt::{Display, Error, Formatter};
use std::result::Result;

use super::color_rgb_ppm::ColorRgbPpm;

#[derive(Clone, Copy)]
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
