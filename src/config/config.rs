use std::env;

pub(crate) struct Config {
    pub image_width: u32,
    pub image_height: u32,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
    }
}
