use serde::Deserialize;

use adapter::repositories::postgres::config::DBConfig;
use common::options::{default_log, Log};

/// Configuration options for the application.
///
/// This struct represents the configuration options for the application, including server settings,
/// database configuration, endpoint for the exporter, service name, and logging configuration.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the server.
    pub server: Server,
    /// Specifies the backend database will be used.
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
    pub pg: Option<DBConfig>,
}

/// Represents in-memory database configuration.
#[derive(Deserialize, Debug)]
pub struct InMemoryDatabase {}

/// Represents server configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port number for the server.
    pub port: u16,
    /// URL for the server.
    pub url: String,
}
