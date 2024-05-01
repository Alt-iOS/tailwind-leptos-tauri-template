use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
	metrics::{
		reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
		MeterProviderBuilder, PeriodicReader, SdkMeterProvider,
	},
	runtime,
	trace::{BatchConfig, RandomIdGenerator, Sampler, Tracer},
	Resource,
};
use opentelemetry_semantic_conventions::{
	resource::{DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION},
	SCHEMA_URL,
};

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
	Resource::from_schema_url(
		[
			KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
			KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
			KeyValue::new(DEPLOYMENT_ENVIRONMENT, "develop"),
		],
		SCHEMA_URL,
	)
}

// Construct MeterProvider for MetricsLayer
pub fn init_meter_provider() -> SdkMeterProvider {
	let exporter = opentelemetry_otlp::new_exporter()
		.tonic()
		.with_endpoint("http://localhost:4317")
		.with_timeout(std::time::Duration::from_secs(2))
		.build_metrics_exporter(
			Box::new(DefaultAggregationSelector::new()),
			Box::new(DefaultTemporalitySelector::new()),
		)
		.unwrap();

	let reader = PeriodicReader::builder(exporter, runtime::Tokio)
		.with_interval(std::time::Duration::from_secs(15))
		.build();

	// For debugging in development
	let stdout_reader =
		PeriodicReader::builder(opentelemetry_stdout::MetricsExporter::default(), runtime::Tokio)
			.build();

	let meter_provider = MeterProviderBuilder::default()
		.with_resource(resource())
		.with_reader(reader)
		.with_reader(stdout_reader)
		.build();

	global::set_meter_provider(meter_provider.clone());

	meter_provider
}

// Construct Tracer for OpenTelemetryLayer
pub fn init_tracer() -> Tracer {
	opentelemetry_otlp::new_pipeline()
		.tracing()
		.with_trace_config(
			opentelemetry_sdk::trace::Config::default()
				.with_sampler(Sampler::AlwaysOn)
				.with_id_generator(RandomIdGenerator::default())
				.with_resource(resource()),
		)
		.with_batch_config(BatchConfig::default())
		.with_exporter(
			opentelemetry_otlp::new_exporter()
				.tonic()
				.with_endpoint("http://localhost:4317")
				.with_timeout(std::time::Duration::from_secs(2)),
		)
		.install_batch(runtime::Tokio)
		.unwrap()
}

pub struct OtelGuard {
	pub meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
	fn drop(&mut self) {
		if let Err(err) = self.meter_provider.shutdown() {
			eprintln!("{err:?}");
		}
		opentelemetry::global::shutdown_tracer_provider();
	}
}
