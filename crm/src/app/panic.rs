use std::any::Any;
use std::panic::{self, PanicInfo};

use backtrace::Backtrace;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::app::config::{Config, Runtime};

// Used for sending panic from Tokio task to main thread
pub type PanicReceiver = Receiver<Box<dyn Any + Send + 'static>>;
pub type PanicSender = Sender<Box<dyn Any + Send + 'static>>;

/// Setup custom panic hook to log error to tracing
pub fn setup_panic_hook(config: &Config) {
    let previous_panic_hook = panic::take_hook();
    let Runtime {
        call_old_panic_hook,
        ..
    } = config.runtime;
    let panic_to_tracing = move |info: &PanicInfo<'_>| {
        tracing::error!(
            target: "panic",
            "thread '{}' {}\nstack backtrace:\n{:?}",
            std::thread::current().name().unwrap_or("<unnamed>"),
            info,
            Backtrace::new(),
        );
        // If true, previous panic hook will be called (usually default panic hook).
        if call_old_panic_hook {
            previous_panic_hook(info);
        }
    };
    panic::set_hook(Box::new(panic_to_tracing));
}
