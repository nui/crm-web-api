use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::{EnvFilter, ParseError};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::prelude::*;

use crate::app::config::Tracing;
use crate::app::is_debug_build;

const DEFAULT_LOG_LEVEL: &str = "info";

#[must_use = "Box<[WorkerGuard]> must be assigned to a binding that is not _"]
pub fn init(config: &Tracing) -> Box<[WorkerGuard]> {
    let mut worker_guards = Vec::new();
    let timer = ChronoLocal::with_format("[%Y-%m-%d %H:%M:%S%.3f %z]".to_owned());

    let stdout_layer = if config.stdout {
        let layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_timer(timer.clone());
        Some(layer)
    } else {
        eprintln!(">> Logging to stdout is disabled by configuration");
        None
    };

    // We usually want daily rotation. If we want more control over rotation, consider disable
    // application level rotation then use external log rotation service.
    let rotation = if config.rotation {
        Rotation::DAILY
    } else {
        Rotation::NEVER
    };

    let file_layer: Option<_> = config.file.as_deref().map(|path| {
        let file_appender = build_rolling_file_appender(rotation.clone(), path);
        let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
        worker_guards.push(worker_guard);
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_timer(timer.clone())
            .with_writer(non_blocking)
    });

    let json_layer: Option<_> = config.json.as_deref().map(|path| {
        let file_appender = build_rolling_file_appender(rotation, path);
        let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
        worker_guards.push(worker_guard);
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_timer(timer.clone())
            .with_writer(non_blocking)
            .json()
    });

    let env_filter = build_env_filter(config).expect("Failed to build EnvFilter");
    // We are setting up logging so we can't use tracing::* macros here.
    eprintln!(">> EnvFilter filters = {}", env_filter);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .with(json_layer)
        .init();
    worker_guards.into_boxed_slice()
}

fn build_rolling_file_appender(rotation: Rotation, path: &Path) -> RollingFileAppender {
    let parent = path.parent().expect("Failed to find log directory");
    let file_name_prefix = path.components().last().expect("Failed to find log name");
    RollingFileAppender::new(rotation, parent, file_name_prefix)
}

fn build_env_filter(config: &Tracing) -> Result<EnvFilter, ParseError> {
    const PREFIX: &str = ">> EnvFilter";
    let env_name = EnvFilter::DEFAULT_ENV;
    if let Ok(directives) = std::env::var(env_name) {
        eprintln!(
            "{} use filters from {} environment variable",
            PREFIX, env_name
        );
        EnvFilter::try_new(directives)
    } else if let Some(ref filters) = config.filters {
        eprintln!("{} use filters from configuration file", PREFIX);
        filters
            .iter()
            .try_fold(EnvFilter::from(DEFAULT_LOG_LEVEL), |acc, directive| {
                Ok(acc.add_directive(directive.parse()?))
            })
    } else {
        eprintln!("{} use default filters", PREFIX);
        default_env_filter()
    }
}

fn default_env_filter() -> Result<EnvFilter, ParseError> {
    let directives = if is_debug_build() {
        "debug,hyper=info,reqwest=info"
    } else {
        DEFAULT_LOG_LEVEL
    };
    EnvFilter::try_new(directives)
}
