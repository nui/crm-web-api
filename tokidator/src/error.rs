use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    SignatureVerificationFail,
    BadAccessTokenEncoding,
    BadSignedMessageEncoding,
    Forbidden,
    ExpiredAccessToken,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
