use std::any::Any;
use std::future::Future;
use std::io;

use chrono::Utc;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinHandle;

use crate::app::config::Config;
use crate::app::panic::PanicSender;

pub fn build(config: &Config) -> io::Result<Runtime> {
    Builder::new_multi_thread()
        .worker_threads(config.runtime.workers)
        .enable_all()
        .build()
}

/// Spawn a new async task that handle panic by sending it to sender
///
/// The corresponding receiver is main thread, it will unwind panic and cause process to exit.
pub fn spawn_task_handle_panic<T>(task: T, sender: &PanicSender)
where
    T: Future<Output = ()> + Send + 'static,
{
    let join_handle = tokio::spawn(task);
    tokio::spawn(handle_task_panic(join_handle, sender.clone()));
}

const FAIL_SENDING_PANIC_EXIT_CODE: i32 = 11;

/// This function must be *PANIC FREE*. We don't want Tokio to capture our task panic.
/// Unrecoverable error must terminate program by `abort` or `exit`.
async fn handle_task_panic(join_handle: JoinHandle<()>, sender: PanicSender) {
    if let Err(join_err) = join_handle.await {
        // we care about panic task only, do nothing if task was cancelled.
        if let Ok(panic) = join_err.try_into_panic() {
            if let Err(err) = sender.send(panic).await {
                // Receiver has been closed, definitely our bug.
                report_error_and_exit(err);
            }
        }
    }
}

fn report_error_and_exit(err: SendError<Box<dyn Any + Send>>) -> ! {
    let msg = format!("{} ERROR: Failed to send panic: {}", Utc::now(), err);
    // NOTE:
    //  This log is unlikely to be logged because we use buffered logging.
    //  The non blocking log WorkerGuard won't have chance to flush log because we terminate
    //  program at the end of this function.
    tracing::error!("{}", msg);
    eprintln!("{}", msg);
    // Exit program immediately. Anything that rely on destructor won't function properly.
    std::process::exit(FAIL_SENDING_PANIC_EXIT_CODE);
}
