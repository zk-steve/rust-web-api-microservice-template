/// Main module for running a simple REST server.
///
/// This module provides the entry point for running a simple REST server. It parses command-line
/// arguments, loads configuration options, initializes telemetry, sets up database repositories,
/// and starts the server to handle incoming HTTP requests.
///
/// The server can be configured using a TOML configuration file, with default values provided
/// for missing or unspecified options.
///
/// # Example
///
/// ```rust
/// use my_server_module::main;
///
/// // Start the server with default configuration
/// main();
/// ```
use openssl;

#[macro_use]
use diesel;

mod options;
mod telemetry;

use crate::options::Options;
use crate::telemetry::init_telemetry;

use clap::{Parser, Subcommand};

use deadpool_diesel::postgres::{Pool, Runtime};
use deadpool_diesel::Manager;
use library::adapter::repositories::in_memory::question::QuestionInMemoryRepository;

use library::core::ports::question::QuestionPort;
use library::routes::Router;
use opentelemetry::global;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;

use library::adapter::repositories::postgres::question_db::QuestionDBRepository;
use tracing::{error, info};

/// Simple REST server.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Print config
    Config,
}

/// Entry point for running the server.
#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.version == true {
        println!(env!("APP_VERSION"));
        return;
    }

    let options = match Options::new(args.config_path) {
        Ok(options) => options,
        Err(err) => {
            println!("Failed to load config: {}", err);
            return;
        }
    };

    if let Some(Commands::Config) = args.command {
        println!("{:#?}", options);
        return;
    }

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let question_port: Arc<dyn QuestionPort + Send + Sync> = if options.db.in_memory.is_some() {
        info!("Using in-memory database");
        Arc::new(QuestionInMemoryRepository::new())
    } else if options.db.pg.is_some() {
        info!("Using postgres database");
        let database_config = options.db.pg.clone().unwrap();
        let manager = Manager::new(database_config.url, Runtime::Tokio1);
        let pool = Pool::builder(manager)
            .max_size(database_config.max_size)
            .build()
            .unwrap();
        Arc::new(QuestionDBRepository::new(pool))
    } else {
        info!("No database specified, falling back to in-memory");
        Arc::new(QuestionInMemoryRepository::new())
    };

    let router = Router::new(question_port);

    let address = SocketAddrV4::new(
        Ipv4Addr::from_str(options.server.url.as_str()).unwrap(),
        options.server.port,
    );
    warp::serve(router.routes()).run(address).await;
    global::shutdown_tracer_provider();
}
