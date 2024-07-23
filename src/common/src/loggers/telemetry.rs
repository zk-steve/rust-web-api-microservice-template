use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{runtime, trace, Resource};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Initializes OpenTelemetry for tracing in a web service.
///
/// This function initializes OpenTelemetry for distributed tracing in a web service.
///
/// # Arguments
///
/// * `service_name` - A string slice representing the name of the service.
/// * `exporter_endpoint` - A string slice representing the endpoint of the exporter.
/// * `log_level` - A string slice representing the level of log.
///
/// # Panics
///
/// This function will panic if it fails to initialize the tracer.
///
pub fn init_telemetry(service_name: &str, exporter_endpoint: &str, log_level: &str) {
    // Create a gRPC exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(exporter_endpoint);

    // Define a tracer
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace::Config::default().with_resource(Resource::new(vec![
            KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                service_name.to_string(),
            ),
        ])))
        .install_batch(runtime::Tokio)
        .expect("Error: Failed to initialize the tracer.")
        .tracer(service_name.to_string());

    // Define a subscriber
    let subscriber = Registry::default();
    // Level filter layer to filter traces based on level (trace, debug, info, warn, error)
    let level_filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(log_level));
    // Layer for adding our configured tracer
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // Layer for printing spans to stdout
    let formatting_layer = BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout);

    global::set_text_map_propagator(TraceContextPropagator::new());

    subscriber
        .with(level_filter_layer)
        .with(tracing_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init()
}
