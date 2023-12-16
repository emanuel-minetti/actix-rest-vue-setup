use log::LevelFilter;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    logfile_path: String,
    // virtual optional field
    #[serde(default)]
    loglevel: Option<String>,
}

impl Settings {
    pub fn loglevel(&self) -> LevelFilter {
        match &self.loglevel {
            None => LevelFilter::Info,
            Some(loglevel) => LevelFilter::from_str(loglevel).unwrap_or(LevelFilter::Info),
        }
    }

    pub fn logfile_path(&self) -> String {
        
        self.logfile_path.clone()
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("configuration.yaml"),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
