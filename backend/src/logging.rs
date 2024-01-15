use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn install_tracing() -> (WorkerGuard, WorkerGuard) {
    LogTracer::init().unwrap();

    // non blocking writers for tracing
    let file_appender = tracing_appender::rolling::hourly("logs/", "backend.log");
    let (file_writer, guard1) = tracing_appender::non_blocking(file_appender);
    let (stdout_writer, guard2) = tracing_appender::non_blocking(std::io::stdout());

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            app_name.to_string(),
            file_writer,
        ))
        .with(BunyanFormattingLayer::new(
            app_name.to_string(),
            stdout_writer,
        ));

    tracing::subscriber::set_global_default(subscriber).unwrap();

    (guard1, guard2)
}
