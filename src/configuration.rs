#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("configuration.yaml")))
        .build()?;
    settings.try_deserialize::<Settings>()
}
