use crate::color_rgb::color_rgb::ColorRgb;
use core::ops::Deref;
use std::fmt::{Display, Error, Formatter};

pub(crate) struct ColorRgbPpm(pub ColorRgb);

impl Deref for ColorRgbPpm {
    type Target = ColorRgb;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ColorRgbPpm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:3} {:3} {:3}", self.red, self.green, self.blue)
    }
}
