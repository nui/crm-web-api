use std::borrow::Cow;

use serde::Serialize;
use tracing::{error, warn};
use warp::http::header::{HeaderValue, CONTENT_TYPE};
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;
use warp::{reply, Rejection, Reply};

use common::json::ApiJson;

use crate::error::{ErrorCodeMessage, Unspecified, UserFacingError};

pub fn into_api_json<T>() -> impl FnOnce(crate::Result<T>) -> Result<reply::Json, warp::Rejection>
where
    T: Serialize,
{
    move |result: crate::Result<T>| {
        let api_json = result.map_or_else(to_api_json_report_error, ApiJson::ok);
        Ok(reply::json(&api_json))
    }
}

#[allow(dead_code)]
pub fn ok_json_error_response<T>(
) -> impl FnOnce(crate::Result<T>) -> Result<reply::Response, warp::Rejection>
where
    T: Serialize,
{
    move |result: crate::Result<T>| {
        let response = result.map_or_else(to_response_report_error, |v| {
            reply::json(&v).into_response()
        });
        Ok(response)
    }
}

pub fn ok_pretty_json_error_response<T>(
) -> impl FnOnce(crate::Result<T>) -> Result<Response, Rejection>
where
    T: Serialize,
{
    move |result: crate::Result<T>| {
        let response = result
            .and_then(to_pretty_json)
            .unwrap_or_else(to_response_report_error);
        Ok(response)
    }
}

fn to_pretty_json<T: Serialize>(v: T) -> crate::Result<Response> {
    Ok(json_response_from_bytes(serde_json::to_vec_pretty(&v)?))
}

fn json_response_from_bytes(bytes: Vec<u8>) -> Response {
    let mut response = Response::new(Body::from(bytes));
    response
        .headers_mut()
        .append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    response
}

const TARGET: &str = "error";

fn to_api_json_report_error<T>(err: anyhow::Error) -> ApiJson<T> {
    let code: Cow<'static, str>;
    let message: Cow<'static, str>;
    if let Some(err) = err.downcast_ref::<UserFacingError>() {
        code = err.code();
        message = err.message();
        emit_warning(&err.message());
    } else {
        code = Cow::Borrowed("500");
        message = Cow::Borrowed("Internal Server Error");
        emit_error(err);
    }
    ApiJson::<T>::error_builder()
        .code(code)
        .message(message)
        .build()
}

fn to_response_report_error(err: anyhow::Error) -> Response {
    if let Some(err) = err.downcast_ref::<UserFacingError>() {
        emit_warning(&err.message());
    } else if err.downcast_ref::<Unspecified>().is_some() {
    } else {
        emit_error(err);
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

fn emit_warning(msg: &str) {
    warn!(target: TARGET, "{}", msg);
}

fn emit_error(err: anyhow::Error) {
    error!(target: TARGET, "{:#?}", err);
}
