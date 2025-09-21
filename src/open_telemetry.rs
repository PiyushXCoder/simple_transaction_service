use log::Level;
use opentelemetry::global;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator};
use std::str::FromStr;

pub fn get_resource() -> Resource {
    Resource::builder()
        .with_service_name("simple-transaction-service")
        .build()
}

pub fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create OTLP exporter");

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build();

    global::set_tracer_provider(tracer_provider);
}

pub fn init_meter_provider() {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::Delta)
        .build()
        .expect("Failed to create OTLP metric exporter");

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_reader(opentelemetry_sdk::metrics::PeriodicReader::builder(exporter).build())
        .with_resource(get_resource())
        .build();

    global::set_meter_provider(meter_provider);
}

pub fn init_logger_provider() {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create OTLP log exporter");

    let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build();

    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).expect("Failed to set logger");

    let max_level = std::env::var("LOG_LEVEL")
        .ok()
        .and_then(|l| Level::from_str(l.to_lowercase().as_str()).ok())
        .unwrap_or(Level::Info);
    log::set_max_level(max_level.to_level_filter());
}

pub fn init_open_telemetry() {
    init_logger_provider();
    init_tracer();
    init_meter_provider();

    log::info!("OpenTelemetry initialized");
}
