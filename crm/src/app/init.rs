use openssl_probe::init_ssl_cert_env_vars;
use tokio::sync::oneshot::Receiver;
use tracing_appender::non_blocking::WorkerGuard;

use crate::app;
use crate::app::config::Config;
use crate::app::panic::{setup_panic_hook, PanicReceiver, PanicSender};

/// Program initialization
///
/// This function setup thing that doesn't need async runtime
pub fn pre_runtime_initialize() -> InitOutput {
    let cmdline = app::cmdline::from_args();
    let config = Config::try_load(cmdline.config.as_deref()).expect("Unable to load configuration");
    #[cfg(feature = "jemalloc")]
    jemalloc::initialize(&config);
    setup_panic_hook(&config);
    init_ssl_cert_env_vars();
    if cmdline.print_config {
        display_current_configuration(&config);
    }
    let worker_guards = app::tracing::init(&config.tracing);
    let shutdown_signal = register_signal_handler();
    let (panic_sender, panic_receiver) = tokio::sync::mpsc::channel(1);
    InitOutput {
        config,
        panic_receiver,
        panic_sender,
        shutdown_signal,
        worker_guards,
    }
}

fn display_current_configuration(config: &Config) {
    // Using eprintln! is intended. We don't want secrets to be in log file!!!
    eprintln!(">> Current configuration\n{}", config.to_pretty_toml())
}

fn register_signal_handler() -> Receiver<()> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let mut sender = Some(sender);
    let handler = move || {
        if let Some(sender) = sender.take() {
            sender.send(()).expect("Error sending shutdown signal");
        }
    };
    ctrlc::set_handler(handler).expect("Error setting signal handler");
    receiver
}

pub struct InitOutput {
    pub config: Config,
    pub panic_receiver: PanicReceiver,
    pub panic_sender: PanicSender,
    pub shutdown_signal: Receiver<()>,
    pub worker_guards: Box<[WorkerGuard]>,
}

#[cfg(feature = "jemalloc")]
mod jemalloc {
    use std::convert::TryInto;
    use std::os::unix::prelude::CommandExt;
    use std::process::Command;

    use crate::app::config::Jemalloc;

    use super::*;

    const MALLOC_CONF: &str = malloc_conf_env_name();

    pub fn initialize(config: &Config) {
        if let Some(ref jemalloc) = config.jemalloc {
            let initialized = std::env::var_os(MALLOC_CONF).is_some();
            if initialized {
                if is_background_thread_supported() {
                    if let Some(enabled) = jemalloc.background_thread {
                        tikv_jemalloc_ctl::background_thread::write(enabled)
                            .expect("Failed to update background thread configuration");
                    }
                }
            } else {
                let mut jemalloc = jemalloc.clone();
                update_number_of_arenas(config, &mut jemalloc);
                apply_jemalloc_configuration(jemalloc);
            }
        }
    }

    fn apply_jemalloc_configuration(jemalloc: Jemalloc) {
        // Some configuration of jemalloc need to be configured before main program is started.
        // But at this point, main program has been started, how do we solve this?
        //
        // We replace current process with itself but with properly jemalloc configuration.

        let malloc_conf = jemalloc.to_config();

        let mut args = std::env::args_os();
        let program = args.next().expect("Program name");
        let mut cmd = Command::new(program);
        cmd.args(args);

        eprintln!(
            ">> Restarting with jemalloc configuration: {}={}",
            MALLOC_CONF, &malloc_conf
        );
        cmd.env(MALLOC_CONF, malloc_conf);
        cmd.exec();
    }

    fn update_number_of_arenas(config: &Config, jemalloc: &mut Jemalloc) {
        jemalloc.number_of_arenas.get_or_insert_with(|| {
            // jemalloc default number of arenas is number of cpu cores * 4
            let num_cores = config.runtime.workers.min(num_cpus::get());
            num_cores
                .checked_mul(4)
                .and_then(|n| n.try_into().ok())
                .expect("Incorrect number of arenas")
        });
    }

    /// Get name of `MALLOC_CONF` env
    const fn malloc_conf_env_name() -> &'static str {
        if is_unprefixed_malloc_supported() {
            "MALLOC_CONF"
        } else {
            "_RJEM_MALLOC_CONF"
        }
    }

    const fn is_background_thread_supported() -> bool {
        // background thread on MacOS is not supported.
        // see https://github.com/jemalloc/jemalloc/issues/843
        !cfg!(target_os = "macos")
    }

    const fn is_unprefixed_malloc_supported() -> bool {
        !cfg!(target_os = "macos")
    }
}
