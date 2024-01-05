use log::LevelFilter;
use serde::Serialize;
use std::str::FromStr;
use std::string::ToString;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application_port: u16,
    pub client_settings: ClientSettings,
    #[serde(default)]
    log_settings: Option<LogSettings>,
}

#[derive(serde::Deserialize, Clone)]
pub struct LogSettings {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    number: Option<u32>,
}

#[derive(serde::Deserialize, Serialize, Clone)]
pub struct ClientSettings {
    copyright: String,
    version: String,
    global_message: String,
}

impl Settings {
    pub fn log_settings(&self) -> LogSettings {
        match &self.log_settings {
            None => LogSettings::new(None, None, None, None),
            Some(log_settings) => log_settings.clone(),
        }
    }
    // TODO move to test
    pub fn set_log_settings(&mut self, log_settings: LogSettings) {
        self.log_settings = Some(log_settings);
    }
}

impl LogSettings {
    pub fn level(&self) -> LevelFilter {
        match &self.level {
            None => LevelFilter::Info,
            Some(loglevel) => LevelFilter::from_str(loglevel).unwrap_or(LevelFilter::Info),
        }
    }
    pub fn path(&self) -> String {
        match &self.path {
            None => "log/logfile".to_string(),
            Some(path) => path.clone(),
        }
    }
    pub fn set_path(&mut self, new_path: String) {
        self.path = Some(new_path)
    }
    pub fn size(&self) -> u64 {
        self.size.unwrap_or(104857600) // 100 MiB
    }
    pub fn number(&self) -> u32 {
        self.number.unwrap_or(9)
    }
    fn new(
        path: Option<String>,
        level: Option<String>,
        size: Option<u64>,
        number: Option<u32>,
    ) -> Self {
        Self {
            path,
            level,
            size,
            number,
        }
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
