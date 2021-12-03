use core::ops::Deref;
use std::fmt::{Display, Error, Formatter};
use std::result::Result;

#[derive(Clone, Copy)]
pub(crate) struct ColorRgb {
    red: u8,
    green: u8,
    blue: u8,
}

pub(crate) struct ColorRgbPpm(ColorRgb);

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
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)
    }
}

impl Deref for ColorRgbPpm {
    type Target = ColorRgb;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ColorRgbPpm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
