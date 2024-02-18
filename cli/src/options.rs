use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Serialize, Deserialize)]
pub struct Options {
    pub web_url: String,
    pub pg: deadpool_postgres::Config,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            web_url: "0.0.0.0:8080".into(),
            pg: deadpool_postgres::Config::default(),
        }
    }
}

impl Options {
    pub fn new(config_file: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(config_file).required(true))
            .build()?;

        s.try_deserialize()
    }
}
