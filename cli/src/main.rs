mod options;

use crate::options::Options;
use clap::Parser;

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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    Options::new(args.config_file.as_str()).expect("TODO: panic message");
}
