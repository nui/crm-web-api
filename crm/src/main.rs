#[macro_use]
mod _macros;

mod api;
mod app;
mod auth;
mod db;
mod error;
mod protos;
mod routes;
mod utils;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Top level type alias for Result type
pub type Result<T, E = anyhow::Error> = core::result::Result<T, E>;

/// The main entrypoint for this application
fn main() -> Result<()> {
    let app::init::InitOutput {
        config,
        panic_receiver,
        panic_sender,
        shutdown_signal,
        worker_guards,
    } = app::init::pre_runtime_initialize();
    let runtime = app::runtime::build(&config)?;
    runtime.block_on(async move {
        let context = app::context::Context::create(config, panic_sender)
            .await
            .into_ref_context();
        app::tasks::register(context);
        app::server::run(context, shutdown_signal, panic_receiver).await
    });
    // This drop call guarantee WorkerGuard are not dropped earlier.
    // See https://docs.rs/tracing-appender/0.1.2/tracing_appender/non_blocking/index.html
    drop(worker_guards);
    runtime.shutdown_background();
    Ok(())
}
