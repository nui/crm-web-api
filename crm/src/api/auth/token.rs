use std::ops::Add;

use chrono::{Duration, Utc};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, instrument};
use warp::{reply, Rejection};

use tokidator::message::SignedMessage;
use tokidator::token::PolicyAccessToken;

use crate::api::auth::utils::policy::json_array_to_policy_set;
use crate::api::warp_helpers::into_api_json;
use crate::app::context::{Context, RefContext};
use crate::auth::rbac::AccessToken;
use crate::db::account::Account;
use crate::error::UserFacingError;

#[derive(Debug, Deserialize)]
pub struct ParseRequest {
    token: String,
}

pub async fn parse(body: ParseRequest, context: RefContext) -> Result<reply::Json, Rejection> {
    parse_impl(body, context).map(into_api_json()).await
}

#[instrument(skip(body, context))]
async fn parse_impl(body: ParseRequest, context: RefContext) -> crate::Result<Value> {
    let Context { public_key, .. } = &*context;
    if let Some(sm) = SignedMessage::decode(&body.token) {
        if sm.verify(public_key) {
            if let Ok(access_token) = AccessToken::from_bytes(sm.message()) {
                return Ok(access_token.to_json());
            }
        }
    }
    Err(UserFacingError::BadAccessToken.into())
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub token: String,
}

pub async fn refresh(token: AccessToken, context: RefContext) -> Result<reply::Json, Rejection> {
    refresh_impl(token, context).map(into_api_json()).await
}

#[instrument(
    name = "refresh-access-token",
    skip(access_token, context),
    fields(access_token.account_id))
]
async fn refresh_impl(
    access_token: AccessToken,
    context: RefContext,
) -> crate::Result<RefreshTokenResponse> {
    debug!("Refreshing access token");
    let Context {
        config,
        pool,
        private_key,
        ..
    } = context.as_ref();
    let account_id = access_token.account_id;
    let login_expired_time = access_token
        .last_login()
        .add(Duration::seconds(config.auth.max_login_secs.into()));
    // force user to login
    if Utc::now() >= login_expired_time {
        return Err(tokidator::Error::ExpiredAccessToken.into());
    }
    // rebuild policies from database
    let Account {
        account_id,
        roles,
        policies,
        ..
    } = crate::db::account::get_by_id(pool, account_id).await?;
    let ps = json_array_to_policy_set(account_id, &roles, &policies)?;
    let next_access_token =
        access_token.refresh(ps, Duration::seconds(config.auth.token_ttl.into()));

    Ok(RefreshTokenResponse {
        token: SignedMessage::create(next_access_token.to_bytes(), private_key).encode(),
    })
}
