use std::borrow::Cow;
use std::fmt::{self, Display};

use thiserror::Error;

use crate::error::ErrorCodeMessage;

/// An error which user should see in standard json response
#[derive(Debug, Error)]
pub enum UserFacingError {
    InvalidUserNameOrPassword,
    UsernameAlreadyExist,
    IllegalUserName,
    IllegalPassword,
    BadAccessToken,
}

impl Display for UserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

const STANDARD_USER_ERROR_CODE: Cow<'static, str> = Cow::Borrowed("400");

impl ErrorCodeMessage for UserFacingError {
    fn code(&self) -> Cow<'static, str> {
        use UserFacingError::*;
        match *self {
            InvalidUserNameOrPassword => "1001".into(),
            UsernameAlreadyExist => "1002".into(),
            IllegalUserName => "1003".into(),
            IllegalPassword => "1004".into(),
            BadAccessToken => STANDARD_USER_ERROR_CODE,
        }
    }

    fn message(&self) -> Cow<'static, str> {
        use UserFacingError::*;
        match *self {
            InvalidUserNameOrPassword => "Invalid username or password".into(),
            UsernameAlreadyExist => "Username already exist".into(),
            IllegalUserName => "Illegal user name".into(),
            IllegalPassword => "Illegal password".into(),
            BadAccessToken => "Bad access token".into(),
        }
    }
}
