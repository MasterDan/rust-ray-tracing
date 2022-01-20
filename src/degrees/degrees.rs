use core::f64::consts::PI;

pub(crate) struct Degrees(pub f64);

impl Degrees {
    pub fn toRadians(self) -> f64 {
        self.0 * PI / 180.0
    }
}
