use super::size::Size;
use crate::ColorRgb;

pub(crate) struct Ppm {
    pub size: Size,
    pub body: Vec<Vec<ColorRgb>>,
}

impl Ppm {
    pub(crate) fn new(height: u32, width: u32) -> Ppm {
        Ppm {
            size: Size::new(height, width),
            body: vec![],
        }
    }
}
