use serde::Deserialize;

use common::options::{default_log, Log};

/// Configuration options for the application.
///
/// This struct represents the configuration options for the application, including server settings,
/// endpoint for server, endpoint for the exporter, service name, and logging configuration.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the grpc server endpoint.
    pub server_endpoint: String,
    /// The endpoint for the exporter.
    pub exporter_endpoint: String,
    /// The name of the service.
    pub service_name: String,
    /// Configuration for redis.
    pub redis: RedisConfig,
    /// Configuration for logging, including log level.
    #[serde(default = "default_log")]
    pub log: Log,
}

/// Represents redis configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}
