pub fn init_config() -> config::Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();
    return settings;
}
