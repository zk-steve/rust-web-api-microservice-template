mod options;
mod telemetry;

use crate::options::Options;
use crate::telemetry::init_telemetry;

use library::question::{Question, QuestionId};
use library::routes::Router;
use library::store::Store;

use clap::Parser;
use opentelemetry::global;
use std::net::SocketAddrV4;
use std::str::FromStr;
use tracing::error;

/// Simple REST server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file
    #[arg(short, long, default_value = "config/config.app.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let options = Options::new(args.config_file.as_str())
        .map_err(|e| {
            error!("Error occurs {}", e);
        })
        .unwrap();

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
    );

    let store = Store::new();
    for a in 0..100 {
        store
            .add(Question::new(
                QuestionId::from_str(a.to_string().as_str()).unwrap(),
                "title".to_string(),
                "content".to_string(),
                None,
            ))
            .await
            .unwrap();
    }

    let router: Router = Router::new(store);

    let address = SocketAddrV4::from_str(options.web_url.as_str())
        .map_err(|e| {
            error!("Error occurs {}", e);
        })
        .unwrap();
    warp::serve(router.routes()).run(address).await;
    global::shutdown_tracer_provider();
}
