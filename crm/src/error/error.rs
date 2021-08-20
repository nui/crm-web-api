use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

pub struct Unspecified;

pub trait ErrorCodeMessage: Debug {
    fn code(&self) -> Cow<'static, str>;
    fn message(&self) -> Cow<'static, str>;
}

impl Display for Unspecified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unspecified")
    }
}

impl Debug for Unspecified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Unspecified {}
