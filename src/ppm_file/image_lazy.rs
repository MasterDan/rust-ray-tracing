use crate::color_rgb::ColorRgb;
use crate::ppm_file::size::Size;
use crate::PpmImage;
use rayon::prelude::*;

pub(crate) enum LazyColor {
    Position(u32, u32),
    Color(ColorRgb),
}

impl LazyColor {
    pub fn calc<T: Fn(u32, u32) -> ColorRgb>(&mut self, init: T) {
        if let LazyColor::Position(row, col) = *self {
            *self = LazyColor::Color(init(row, col));
        }
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
    pub fn calculate<T>(&mut self, init: T) -> PpmImage
    where
        T: Fn(u32, u32) -> ColorRgb + Send + Sync,
    {
        for row in self.body.iter_mut() {
            row.par_iter_mut()
                .for_each(|v| v.calc(|row, col| init(row, col)))
        }
        self.to_image()
    }

    fn to_image(&self) -> PpmImage {
        let mut body: Vec<Vec<ColorRgb>> = Vec::new();
        for r in self.body.iter() {
            let row = r
                .iter()
                .map(|item| {
                    if let LazyColor::Color(color_rgb) = *item {
                        return color_rgb;
                    } else {
                        panic!("All colors must be counted!");
                    }
                })
                .collect::<Vec<ColorRgb>>();
            body.push(row);
        }
        PpmImage {
            size: Size::new(self.size.height, self.size.width),
            body,
        }
    }
}
