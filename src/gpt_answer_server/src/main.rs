use clap::{Parser, Subcommand};
use opentelemetry::global;
use tonic::transport::Server;

use common::grpc::gpt_answer::gpt_answer::gpt_answer_service_server::GptAnswerServiceServer;
use common::loggers::telemetry::init_telemetry;
use common::options::parse_options;
use gpt_answer_server::controllers::gpt_answer::GptAnswerServiceImpl;
use gpt_answer_server::options::Options;

pub async fn serve(options: Options) {
    let address = options.server_endpoint.parse().unwrap();
    println!("Starting GPT Answer server at {}", options.server_endpoint);

    let gpt_answer_service = GptAnswerServiceImpl::new("dummy_prop".to_string());
    Server::builder()
        .add_service(GptAnswerServiceServer::new(gpt_answer_service))
        .serve(address)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.version {
        println!(env!("APP_VERSION"));
        return;
    }

    let options: Options = match parse_options(args.config_path) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
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

    let server = tokio::spawn(serve(options));

    tokio::try_join!(server).expect("Failed to run servers");

    global::shutdown_tracer_provider();
}

/// GPT Answer GRPC server.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
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
