use once_cell::sync::OnceCell;
use tracing::info;

use crate::app::config::{Config, RequestInfo};

/// Create a request logger closure
///
/// N.B. Output closure will be cloned for every incoming request.
/// Make sure that it is cheap to clone this closure.
pub fn make_request_logger(config: &Config) -> impl Fn(warp::log::Info) + Clone {
    let Config { request_info, .. } = config;
    static SKIP_PATHS: OnceCell<Box<[Box<str>]>> = OnceCell::new();
    // Coerce to static reference so it is cheap to clone
    let skip_paths: &'static [Box<str>] = SKIP_PATHS.get_or_init(|| make_skip_paths(request_info));
    let emit = request_info.emit;
    move |info| {
        if !emit {
            return;
        }
        let path = info.path();
        if should_log_request_info(path, skip_paths) {
            info!(
                target: "request_info",
                usage = format_args!("{:.3}", info.elapsed().as_secs_f32()),
                "{} {} [{}]",
                info.method(),
                path,
                info.status().as_u16(),
            );
        }
    }
}

fn make_skip_paths(request_info: &RequestInfo) -> Box<[Box<str>]> {
    request_info
        .skip
        .iter()
        .map(|s| s.clone().into_boxed_str())
        .collect()
}

/// Allow disable request logging for some routes
///
/// We don't wont to log all requests, for example, health check request.
fn should_log_request_info(path: &str, skip_paths: &[Box<str>]) -> bool {
    // This can be changed to BTreeSet if a skip path list is large
    let skip = skip_paths.iter().any(|p| p.as_ref() == path);
    !skip
}
