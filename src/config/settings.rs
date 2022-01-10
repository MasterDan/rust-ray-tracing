use crate::config::aspect_ratio_string::AspectRatioString;

pub(crate) struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
}

impl Settings {
    pub fn new() -> Self {
        let config = init_config();
        let width = config.get_int("image_width").unwrap_or(256) as u32;
        let samples_per_pixel = config.get_int("samples_per_pixel").unwrap_or(100) as u32;
        let viewport_height = config.get_float("viewport_height").unwrap_or(2.0);
        let focal_length = config.get_float("focal_length").unwrap_or(1.0);
        let aspect_ratio = AspectRatioString(
            config
                .get_str("aspect_ratio")
                .unwrap_or(String::from("16 / 9")),
        )
        .parse()
        .unwrap();
        Settings {
            image_width: width,
            image_height: (width as f64 / aspect_ratio) as u32,
            viewport_height,
            viewport_width: aspect_ratio * viewport_height,
            focal_length,
            aspect_ratio,
            samples_per_pixel,
        }
    }
}

fn init_config() -> config::Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();
    return settings;
}
