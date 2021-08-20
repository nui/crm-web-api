use futures::FutureExt;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use tracing::{debug, instrument};
use warp::{reply, Rejection};

use crate::api::auth::utils::password::hash_password;
use crate::api::warp_helpers::into_api_json;
use crate::app::context::{Context, RefContext};
use crate::db::account::Account;
use crate::error::UserFacingError;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    name: String,
    password: String,
}

static USERNAME_PATTERN: Lazy<Regex> = Lazy::new(|| {
    let pattern = r#"^[[:alpha:]][[:alnum:]]{2,}$"#;
    Regex::new(pattern).unwrap()
});

impl CreateRequest {
    pub fn validate(&self) -> Result<(), UserFacingError> {
        if !USERNAME_PATTERN.is_match(&self.name) {
            Err(UserFacingError::IllegalUserName)
        } else if self.password.chars().count() <= 3 {
            Err(UserFacingError::IllegalPassword)
        } else {
            Ok(())
        }
    }
}

pub async fn create(body: CreateRequest, context: RefContext) -> Result<reply::Json, Rejection> {
    create_impl(body, context).map(into_api_json()).await
}

#[instrument(name = "create-account", skip(body, context))]
async fn create_impl(body: CreateRequest, context: RefContext) -> crate::Result<bool> {
    body.validate()?;
    let Context { pool, config, .. } = &*context;
    let CreateRequest { name, password } = body;
    debug!("Creating new account, name: {}", name);
    let username_exist = crate::db::account::is_username_exist(pool, &name).await?;
    if username_exist {
        return Err(UserFacingError::UsernameAlreadyExist.into());
    }
    let password_hash = hash_password(password, config.auth.bcrypt_cost).await?;
    let account = Account::new_account(name, password_hash);
    crate::db::account::create(pool, account).await?;
    Ok(true)
}

pub async fn list(context: RefContext) -> Result<reply::Json, Rejection> {
    list_impl(context).map(into_api_json()).await
}

#[instrument(name = "list-account", skip(context))]
async fn list_impl(context: RefContext) -> crate::Result<Vec<Value>> {
    let Context { pool, .. } = &*context;
    let accounts = crate::db::account::list(pool).await?;
    let output_accounts = accounts
        .into_iter()
        .map(|a| {
            serde_json::json!({
                "id": a.account_id,
                "name": a.name,
                "policies": a.policies,
                "created": a.created,
                "allow_login": a.allow_login,
            })
        })
        .collect();
    Ok(output_accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(USERNAME_PATTERN.is_match("abcd"));
        assert!(USERNAME_PATTERN.is_match("y11"));
        assert!(!USERNAME_PATTERN.is_match("x1"));
        assert!(!USERNAME_PATTERN.is_match("1aeoeux"));
        assert!(!USERNAME_PATTERN.is_match("111"));
    }
}
