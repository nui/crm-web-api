pub use error::{ErrorCodeMessage, Unspecified};
pub use user_facing_error::UserFacingError;

#[allow(clippy::module_inception)]
mod error;
mod user_facing_error;
