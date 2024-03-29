use config::Config;

use crate::config::aspect_ratio_string::AspectRatioString;

pub(crate) struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_height: f64,
    pub vfow: f64,
    pub aperture: f64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub dist_to_focus: f64,
}

impl Settings {
    pub fn new() -> Self {
        let config = init_config();
        let width = config.get_int("image_width").unwrap_or(256) as u32;
        let samples_per_pixel = config.get_int("samples_per_pixel").unwrap_or(100) as u32;
        let max_depth = config.get_int("max_depth").unwrap_or(50) as u32;
        let viewport_height = config.get_float("viewport_height").unwrap_or(2.0);
        let dist_to_focus = config.get_float("dist_to_focus").unwrap_or(20.0);
        let vfov = config
            .get_float("vertical_field_of_view_in_degrees")
            .unwrap_or(100.0);
        let aperture = config.get_float("aperture").unwrap_or(2.0);
        let aspect_ratio = AspectRatioString(
            config
                .get_string("aspect_ratio")
                .unwrap_or(String::from("16 / 9")),
        )
        .parse()
        .unwrap();
        Settings {
            image_width: width,
            image_height: (width as f64 / aspect_ratio) as u32,
            viewport_height,
            samples_per_pixel,
            max_depth,
            aspect_ratio,
            vfow: vfov,
            aperture,
            dist_to_focus,
        }
    }
}

fn init_config() -> Config {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .add_source(config::Environment::with_prefix("APP"));
    settings.build().expect("Cannot createConfig")
}
