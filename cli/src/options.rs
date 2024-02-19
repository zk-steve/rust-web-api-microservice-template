use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::Deserialize;

/// Configuration options for the application.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the server.
    pub server: Server,
    /// Configuration for PostgresSQL.
    pub pg: deadpool_postgres::Config,
    /// Endpoint for the exporter.
    pub exporter_endpoint: String,
    /// Name of the service.
    pub service_name: String,
    /// Configuration for logging.
    #[serde(default = "default_log")]
    pub log: Log,
}

/// Configuration for logging.
#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    /// Log level.
    pub level: String,
}

/// Default logging configuration.
fn default_log() -> Log {
    Log {
        level: "INFO".to_string(),
    }
}

/// Configuration for the server.
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port number for the server.
    pub port: u16,
    /// URL for the server.
    pub url: String,
}

impl Options {
    /// Creates a new `Options` instance by loading configurations from specified paths.
    ///
    /// # Arguments
    ///
    /// * `config_path` - A vector of file paths where configuration files are located.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed `Options` instance or a `ConfigError` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use my_app_config::Options;
    ///
    /// let config_paths = vec!["config/*.toml".to_string(), "/etc/my_app/config.yaml".to_string()];
    /// match Options::new(config_paths) {
    ///     Ok(options) => {
    ///         println!("Config loaded successfully: {:?}", options);
    ///     },
    ///     Err(err) => {
    ///         eprintln!("Failed to load config: {}", err);
    ///     }
    /// }
    /// ```
    pub fn new(config_path: Vec<String>) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(
                config_path
                    .iter()
                    .flat_map(|path| {
                        glob(path)
                            .unwrap()
                            .map(|path| File::from(path.unwrap()))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
            .add_source(
                Environment::default()
                    .separator("__")
                    .list_separator(",")
                    .ignore_empty(true),
            )
            .build()?;

        s.try_deserialize()
    }
}
