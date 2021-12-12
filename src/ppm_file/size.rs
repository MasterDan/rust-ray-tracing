use core::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::result::Result;

pub(crate) struct Size {
    pub height: u32,
    pub width: u32,
}

impl Size {
    pub(crate) fn new(height: u32, width: u32) -> Size {
        Size {
            height: height,
            width: width,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.width, self.height)
    }
}
