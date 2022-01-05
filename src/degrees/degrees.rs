use core::f64::consts::PI;

pub(crate) struct Degrees(u32);

impl Degrees {
    pub fn toRadians(self) -> f64 {
        (self.0 as f64) * PI / 180.0
    }
}
