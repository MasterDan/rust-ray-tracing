use crate::color_rgb::ColorRgb;
use crate::ppm_file::size::Size;
use crate::PpmImage;

pub(crate) enum LazyColor {
    Position(u32, u32),
    Processing,
    Color(ColorRgb),
}

impl LazyColor {
    pub fn calc<T: Fn(u32, u32) -> ColorRgb>(&mut self, init: T) {
        if let LazyColor::Position(row, col) = *self {
            *self = LazyColor::Processing;
            *self = LazyColor::Color(init(row, col));
        }
        todo!();
    }
}

pub(crate) struct PpmImageLazy {
    pub size: Size,
    pub body: Vec<Vec<LazyColor>>,
}

impl PpmImageLazy {
    pub fn new(height: u32, width: u32) -> Self {
        let mut body: Vec<Vec<LazyColor>> = Vec::new();
        for r in 0..height {
            let mut row: Vec<LazyColor> = Vec::new();
            for c in 0..width {
                row.push(LazyColor::Position(r, c))
            }
            body.push(row);
        }
        PpmImageLazy {
            size: Size::new(height, width),
            body,
        }
    }
    pub fn calc<T: Fn(u32, u32) -> ColorRgb>(&mut self, init: T) -> PpmImage {
        todo!()
    }
}
