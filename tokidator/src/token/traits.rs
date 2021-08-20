use std::ops::Deref;

use crate::rbac::{Policy, PolicySet};

pub trait PolicyAccessToken: Sized {
    type Policy: Policy;
    type Error;

    fn policies(&self) -> &PolicySet<Self::Policy>;
    fn is_expired(&self) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Result<Self, Self::Error>;
}

pub trait ToTokenStr {
    fn to_token_str(&self) -> Option<&str>;
}

impl<T: Deref<Target = str>> ToTokenStr for Option<T> {
    fn to_token_str(&self) -> Option<&str> {
        self.as_deref()
    }
}
