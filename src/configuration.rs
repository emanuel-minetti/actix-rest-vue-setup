use log::LevelFilter;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub log_settings: LogSettings,
}

#[derive(serde::Deserialize)]
pub struct LogSettings{
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    number: Option<u32>,
}

impl LogSettings {
    pub fn level(&self) -> LevelFilter {
        match &self.level {
            None => LevelFilter::Info,
            Some(loglevel) => LevelFilter::from_str(loglevel).unwrap_or(LevelFilter::Info),
        }
    }

    pub fn path(&self) -> String {
        self.path.clone().unwrap_or("log/logfile".to_string())
    }
    pub fn size(&self) -> u64 {
        self.size.unwrap_or(104857600) // 100 MiB
    }
    pub fn number(&self) -> u32 {
        self.number.unwrap_or(9)
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
