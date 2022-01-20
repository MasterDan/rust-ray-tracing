use crate::color_rgb::ColorRgb;
use crate::ppm_file::size::Size;
use crate::PpmImage;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;

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
pub(crate) struct ImageLazyCounted(pub ImageLazy);

pub(crate) struct ImageLazy {
    pub size: Size,
    pub body: Vec<Vec<LazyColor>>,
}

impl ImageLazy {
    pub fn new(height: u32, width: u32) -> Self {
        let mut body: Vec<Vec<LazyColor>> = Vec::new();
        for r in 0..height {
            let mut row: Vec<LazyColor> = Vec::new();
            for c in 0..width {
                row.push(LazyColor::Position(r, c))
            }
            body.push(row);
        }
        ImageLazy {
            size: Size::new(height, width),
            body,
        }
    }
    pub fn calculate<T>(mut self, init: T) -> ImageLazyCounted
    where
        T: Fn(u32, u32) -> ColorRgb + Send + Sync,
    {
        for row in self.body.iter_mut() {
            row.par_iter_mut()
                .for_each(|v| v.calc(|row, col| init(row, col)))
        }
        ImageLazyCounted(self)
    }
}

impl ImageLazyCounted {
    pub fn to_image_ppm(&self) -> PpmImage {
        let mut body: Vec<Vec<ColorRgb>> = Vec::new();
        for r in self.0.body.iter() {
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
            size: Size::new(self.0.size.height, self.0.size.width),
            body,
        }
    }
    pub fn save_as_png(&self, file: File) {
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.0.size.width, self.0.size.height);
        let mut data: Vec<u8> =
            Vec::with_capacity((self.0.size.width * self.0.size.height) as usize);
        for r in self.0.body.iter() {
            for c in r.iter() {
                if let LazyColor::Color(color_rgb) = c {
                    data.push(color_rgb.red);
                    data.push(color_rgb.green);
                    data.push(color_rgb.blue);
                    data.push(255);
                } else {
                    panic!("All colors must be counted!");
                }
            }
        }

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap();
    }
}
