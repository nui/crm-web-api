use chrono::{FixedOffset, SecondsFormat, TimeZone};
use futures::FutureExt;
use serde::Serialize;
use warp::reply::Response;
use warp::Rejection;

use crate::api::warp_helpers::ok_pretty_json_error_response;

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    pub build_on: String,
    pub git_head: Option<&'static str>,
    pub version: &'static str,
    pub profile: &'static str,
    pub toolchain: Toolchain,
}

#[derive(Debug, Serialize)]
pub struct Toolchain {
    pub rustc: &'static str,
    pub target: &'static str,
}

pub fn build_time() -> i64 {
    env!("BUILD_EPOCHSECONDS")
        .parse()
        .expect("Fail to get build_time")
}

pub async fn info() -> Result<Response, Rejection> {
    info_impl().map(ok_pretty_json_error_response()).await
}

async fn info_impl() -> crate::Result<InfoResponse> {
    let thailand_offset = FixedOffset::east(7 * 60 * 60);
    let build_on = thailand_offset
        .timestamp(build_time(), 0)
        .to_rfc3339_opts(SecondsFormat::Secs, true);

    Ok(InfoResponse {
        build_on,
        git_head: option_env!("BUILD_COMMIT_ID"),
        version: env!("CARGO_PKG_VERSION"),
        profile: env!("BUILD_PROFILE"),
        toolchain: Toolchain {
            rustc: env!("BUILD_RUSTC_VERSION"),
            target: env!("BUILD_TARGET"),
        },
    })
}
