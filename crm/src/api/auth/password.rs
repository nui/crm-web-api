use futures::FutureExt;
use serde::Deserialize;
use tracing::instrument;
use warp::{reply, Rejection};

use crate::api::auth::utils::password::{get_junk_hash_duration, hash_password, verify_password};
use crate::api::warp_helpers::into_api_json;
use crate::app::context::{Context, RefContext};
use crate::db::account::Account;
use crate::error::UserFacingError;

#[derive(Debug, Deserialize)]
pub struct EncryptPasswordRequest {
    password: String,
    cost: Option<u32>,
}

pub async fn encode(
    body: EncryptPasswordRequest,
    context: RefContext,
) -> Result<reply::Json, Rejection> {
    encode_impl(body, context.config.auth.bcrypt_cost)
        .map(into_api_json())
        .await
}

#[instrument(name = "encode-password", skip(body))]
async fn encode_impl(body: EncryptPasswordRequest, default_cost: u32) -> crate::Result<String> {
    let EncryptPasswordRequest { password, cost } = body;
    let password_hash = hash_password(password, cost.unwrap_or(default_cost)).await?;
    Ok(password_hash)
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    username: String,
    current_password: String,
    new_password: String,
}

pub async fn change(
    input: ChangePasswordRequest,
    context: RefContext,
) -> Result<reply::Json, Rejection> {
    change_impl(input, context).map(into_api_json()).await
}

#[instrument(name = "change-password", skip(input, context))]
async fn change_impl(input: ChangePasswordRequest, context: RefContext) -> crate::Result<bool> {
    let Context { pool, config, .. } = &*context;
    let ChangePasswordRequest {
        username,
        current_password,
        new_password,
    } = input;
    let account_opt = crate::db::account::find_allowed_login_by_name(pool, &username).await?;
    match account_opt {
        Some(Account {
            account_id,
            password_hash,
            ..
        }) => {
            let authenticated = verify_password(current_password, password_hash).await?;
            if authenticated {
                let new_hash = hash_password(new_password, config.auth.bcrypt_cost).await?;
                crate::db::account::change_password(pool, account_id, new_hash).await?;
                Ok(true)
            } else {
                Err(UserFacingError::InvalidUserNameOrPassword.into())
            }
        }
        None => {
            let duration = get_junk_hash_duration(config.auth.bcrypt_cost).await;
            tokio::time::sleep(duration).await;
            Err(UserFacingError::InvalidUserNameOrPassword.into())
        }
    }
}
