mod options;
mod telemetry;

use crate::telemetry::init_telemetry;
use crate::options::Options;

use clap::Parser;
use opentelemetry::trace::{Tracer};
use tracing::{error};
use opentelemetry::{global};


/// Simple REST server
#[derive(Parser, Debug)]
#[command(name = "rust-server")]
#[command(version = "1.0")]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file
    #[arg(short, long, default_value = "config/config.app.toml")]
    config_file: String,
}

const EXPORTER_ENDPOINT: &str = "http://localhost:7281";
const SERVICE_NAME: &str = "rust-api-server";

#[tokio::main]
async fn main() {
    init_telemetry(SERVICE_NAME, EXPORTER_ENDPOINT);
    global::shutdown_tracer_provider();
    let args = Args::parse();
    Options::new(args.config_file.as_str()).map_err(|_e| {
        error!("error occurs");
    }).unwrap();
}
