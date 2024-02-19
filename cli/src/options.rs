use config::ConfigError::{Message};
use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::Deserialize;

/// Configuration options for the application.
///
/// This struct represents the configuration options for the application, including server settings,
/// database configuration, endpoint for the exporter, service name, and logging configuration.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the server.
    pub server: Server,
    /// Specifies whether to use a database and holds its configuration.
    pub db: Database,
    /// The endpoint for the exporter.
    pub exporter_endpoint: String,
    /// The name of the service.
    pub service_name: String,
    /// Configuration for logging, including log level.
    #[serde(default = "default_log")]
    pub log: Log,
}

/// Represents database configuration options.
#[derive(Deserialize, Debug)]
pub struct Database {
    /// Configuration for using in-memory database.
    pub in_memory: Option<InMemoryDatabase>,
    /// Configuration for PostgresSQL.
    pub pg: Option<deadpool_postgres::Config>,
}

/// Represents in-memory database configuration.
#[derive(Deserialize, Debug)]
pub struct InMemoryDatabase {}

/// Represents logging configuration.
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

/// Represents server configuration.
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
    /// * `config_paths` - A vector of file paths where configuration files are located.
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
    pub fn new(config_paths: Vec<String>) -> Result<Self, ConfigError> {
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
}
