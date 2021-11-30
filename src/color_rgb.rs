use std::fmt::{Display, Error, Formatter};
use std::result::Result;

pub struct ColorRgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRgb {
            red: r,
            green: g,
            blue: b,
        }
    }
}

impl Display for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
