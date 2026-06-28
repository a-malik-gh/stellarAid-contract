use std::env;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)))
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .without_time(),
        );

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| "logging already initialized".into())
}
