use core::fmt::Display;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.width, self.height)
    }
}
