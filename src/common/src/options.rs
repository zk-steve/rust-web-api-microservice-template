use config::ConfigError::Message;
use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::Deserialize;

pub fn parse_options<'de, T: Deserialize<'de>>(
    config_paths: Vec<String>,
) -> Result<T, ConfigError> {
    let mut config = Config::builder();

    for path in &config_paths {
        let paths = glob(path).map_err(|e| Message(e.to_string()))?;
        for entry in paths {
            let entry = entry.map_err(|e| Message(e.to_string()))?;
            config = config.add_source(File::from(entry));
        }
    }

    let config = config
        .add_source(
            Environment::default()
                .separator("__")
                .list_separator(",")
                .ignore_empty(true),
        )
        .build()?;

    config.try_deserialize()
}

/// Represents logging configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    /// Log level.
    pub level: String,
}

/// Default logging configuration.
pub fn default_log() -> Log {
    Log {
        level: "INFO".to_string(),
    }
}
