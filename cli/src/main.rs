mod options;
mod telemetry;

use crate::options::Options;
use crate::telemetry::init_telemetry;

use library::question::{Question, QuestionId};
use library::store::Store;

use clap::Parser;
use library::routes::Router;
use opentelemetry::global;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;


/// Simple REST server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file
    #[arg(short, long, default_value = "config/default.toml")]
    config_path: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let options = Options::new(args.config_path)
        .map_err(|e| {
            println!("Error occurs: {}", e);
        })
        .unwrap();

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
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

    let address = SocketAddrV4::new(
        Ipv4Addr::from_str(options.server.url.as_str()).unwrap(),
        options.server.port,
    );
    warp::serve(router.routes()).run(address).await;
    global::shutdown_tracer_provider();
}
