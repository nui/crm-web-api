pub mod cmdline;
pub mod config;
pub mod context;
pub mod init;
pub mod panic;
pub mod rejection;
pub mod runtime;
pub mod server;
pub mod tasks;
pub mod tracing;

pub fn is_debug_build() -> bool {
    cfg!(build = "debug")
}
