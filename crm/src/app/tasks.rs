use crate::app::context::RefContext;
use crate::app::runtime::spawn_task_handle_panic;

/// The main entrypoint for registering async tasks.
pub fn register(context: RefContext) {
    // When there is real task, remove this empty task
    // This is added to keep rust not blaming about unused function
    spawn_task_handle_panic(noop_task(), &context.panic_sender);
}

async fn noop_task() {}
