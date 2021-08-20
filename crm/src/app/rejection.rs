use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt::{self, Debug, Display};

use tracing::{error, warn};
use warp::body::BodyDeserializeError;
use warp::http::header::CONTENT_TYPE;
use warp::http::{HeaderValue, StatusCode};
use warp::hyper::Body;
use warp::reply::Response;
use warp::{reply, Reply};

use common::json::ApiJson;

use crate::error::ErrorCodeMessage;

const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal Server Error";

pub async fn handle_rejection(
    reason: warp::Rejection,
) -> Result<reply::WithStatus<reply::Response>, Infallible> {
    Ok(handle_rejection_sync(reason))
}

/// Convert rejection to http response
fn handle_rejection_sync(reason: warp::Rejection) -> reply::WithStatus<reply::Response> {
    let code: Cow<'static, str>;
    let message: Cow<'static, str>;

    if reason.is_not_found() {
        return reply::with_status(StatusCode::NOT_FOUND.into_response(), StatusCode::NOT_FOUND);
    } else if let Some(err) = reason.find::<BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        code = StatusCode::BAD_REQUEST.as_str().into();
        message = err.to_string().into();
    } else if let Some(err) = reason.find::<TokenError>() {
        code = err.code();
        message = err.message();
        warn!("{:?}", err);
    } else if reason.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED.as_str().into();
        message = "Method Not Allowed".into();
    } else if let Some(e) = reason.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST.as_str().into();
        message = format!("Missing header: {}", e.name()).into();
    } else {
        error!("Unhandled rejection: {:#?}", reason);
        code = StatusCode::INTERNAL_SERVER_ERROR.as_str().into();
        message = INTERNAL_SERVER_ERROR_MESSAGE.into();
    }
    let error_json = ApiJson::<()>::error_builder()
        .code(code)
        .message(message)
        .build();
    match serde_json::to_vec(&error_json) {
        Ok(body) => {
            let mut response = Response::new(Body::from(body));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            reply::with_status(response, StatusCode::OK)
        }
        Err(err) => {
            error!("Error while serializing ApiJson::Error: {:?}", err);
            let response = Response::new(Body::empty());
            reply::with_status(response, StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct TokenError(tokidator::Error);

impl From<tokidator::Error> for TokenError {
    fn from(err: tokidator::Error) -> Self {
        Self(err)
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl warp::reject::Reject for TokenError {}

impl ErrorCodeMessage for TokenError {
    fn code(&self) -> Cow<'static, str> {
        use tokidator::Error::*;
        let code = match self.0 {
            Unauthorized => "401",
            ExpiredAccessToken => "401",
            Forbidden => "403",
            SignatureVerificationFail | BadAccessTokenEncoding | BadSignedMessageEncoding => "400",
        };
        code.into()
    }

    fn message(&self) -> Cow<'static, str> {
        use tokidator::Error::*;
        let message = match self.0 {
            Unauthorized => "Unauthorized, please check your access token.",
            ExpiredAccessToken => "Your access token has expired.",
            Forbidden => "You don't have permission to perform this request.",
            SignatureVerificationFail | BadAccessTokenEncoding | BadSignedMessageEncoding => {
                "Invalid access token."
            }
        };
        message.into()
    }
}
