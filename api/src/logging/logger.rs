use infrastructure::config::{AppConfig, Environment};
use tower_http::{
    LatencyUnit,
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match config.environment {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(subscriber)
        .try_init()?;

    Ok(())
}

pub fn build_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis),
        )
}
