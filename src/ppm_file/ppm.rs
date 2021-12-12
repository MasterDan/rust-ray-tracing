use super::size::Size;
use crate::ColorRgb;
use indicatif::ProgressBar;
use std::fmt::{Display, Error, Formatter};

pub(crate) struct PpmImage {
    pub size: Size,
    pub body: Vec<Vec<ColorRgb>>,
}

impl PpmImage {
    pub(crate) fn new<T>(height: u32, width: u32, init: T) -> PpmImage
    where
        T: Fn(u32, u32) -> ColorRgb,
    {
        let mut body: Vec<Vec<ColorRgb>> = Vec::new();
        for i in 0..width {
            let mut row: Vec<ColorRgb> = Vec::new();
            for j in 0..height {
                row.push(init(i, j));
            }
            body.push(row);
        }
        PpmImage {
            size: Size::new(height, width),
            body: body,
        }
    }
}
impl Display for PpmImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "P3\n")?;
        write!(f, "{}\n", self.size)?;
        write!(f, "{}\n", 255)?;
        let bar = ProgressBar::new((self.size.width * self.size.height).into());
        for (ri, row) in self.body.iter().enumerate() {
            for (ci, color) in row.iter().enumerate() {
                bar.inc(1);
                bar.set_message(format!("Processing {} row {} column...\r", ri, ci));
                write!(f, "{}\n", color.to_ppm_format())?;
            }
        }
        bar.finish_with_message("Done!");
        Ok(())
    }
}
