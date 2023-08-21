use super::size::Size;
use crate::color_rgb::ColorRgb;
use colored::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::fmt::{Display, Error, Formatter};

pub(crate) struct PpmImage {
    pub size: Size,
    pub body: Vec<Vec<ColorRgb>>,
}

impl Display for PpmImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "P3\n")?;
        write!(f, "{}\n", self.size)?;
        write!(f, "{}\n", 255)?;
        let full_size = self.size.width * self.size.height;
        let bar = ProgressBar::new((full_size).into());
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} | {elapsed_precise} | {bar:50} | {msg}")
                .expect("Cannot create progress bar")
                .progress_chars("##-"),
        );
        for (ri, row) in self.body.iter().enumerate() {
            for (ci, color) in row.iter().enumerate() {
                bar.inc(1);
                let percents =
                    100.0 * (ri * (self.size.width as usize) + ci) as f64 / full_size as f64;
                bar.set_message(format!("Processing | {:.1} %\r", percents));
                write!(f, "{}\n", color.to_ppm_format())?;
            }
        }
        bar.finish_with_message(format!("{}", "Done!".green()));
        Ok(())
    }
}
