use futures::FutureExt;
use serde::Serialize;
use warp::reply::Response;
use warp::Rejection;

use crate::api::warp_helpers::ok_pretty_json_error_response;

pub async fn disable_background_thread() -> Result<Response, Rejection> {
    change_background_thread(false)
        .map(ok_pretty_json_error_response())
        .await
}

pub async fn enable_background_thread() -> Result<Response, Rejection> {
    change_background_thread(true)
        .map(ok_pretty_json_error_response())
        .await
}

#[cfg(not(feature = "jemalloc"))]
async fn change_background_thread(enable: bool) -> crate::Result<BackgroundThread> {
    let _ = enable;
    tracing::error!("Compile without jemalloc");
    Err(crate::error::Unspecified.into())
}

#[cfg(feature = "jemalloc")]
async fn change_background_thread(enable: bool) -> crate::Result<BackgroundThread> {
    use tokio::task::spawn_blocking;
    let tag = if enable {
        "enable jemalloc background thread"
    } else {
        "disable jemalloc background thread"
    };
    let blocking = move || {
        measure_time!(tag, {
            let background_thread = tikv_jemalloc_ctl::background_thread::mib()?;
            background_thread.write(enable)?;
            background_thread.read()
        })
    };
    let enabled = spawn_blocking(blocking).await.expect("task not fail")?;
    Ok(BackgroundThread { enabled })
}

#[derive(Serialize)]
struct BackgroundThread {
    enabled: bool,
}
