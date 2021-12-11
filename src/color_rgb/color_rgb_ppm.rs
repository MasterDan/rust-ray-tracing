use super::ColorRgb;
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
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
