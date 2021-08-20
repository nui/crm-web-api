use chrono::{Duration, FixedOffset, SecondsFormat};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use warp::{reply, Rejection};

use tokidator::message::SignedMessage;
use tokidator::token::PolicyAccessToken;

use crate::api::auth::utils::password::{get_junk_hash_duration, verify_password};
use crate::api::auth::utils::policy::json_array_to_policy_set;
use crate::api::warp_helpers::into_api_json;
use crate::app::context::{Context, RefContext};
use crate::auth::rbac::AccessToken;
use crate::db::account::Account;
use crate::error::UserFacingError;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResponse {
    pub id: i64,
    pub token: String,
    pub not_after: String,
}

pub async fn sign_in(input: SignInRequest, context: RefContext) -> Result<reply::Json, Rejection> {
    sign_in_impl(context, input).map(into_api_json()).await
}

#[instrument(
    name = "sign-in",
    skip(context, input),
    fields(username=input.username.as_str()))
]
async fn sign_in_impl(context: RefContext, input: SignInRequest) -> crate::Result<SignInResponse> {
    let Context {
        pool,
        private_key,
        config,
        ..
    } = context.as_ref();
    let SignInRequest { username, password } = input;
    if let Some(account) = crate::db::account::find_allowed_login_by_name(pool, &username).await? {
        let Account {
            account_id,
            password_hash,
            roles,
            policies,
            ..
        } = account;
        let authenticated = measure_time!(
            "verify password",
            verify_password(password, password_hash).await
        )?;
        if authenticated {
            let ps = json_array_to_policy_set(account_id, &roles, &policies)?;
            let access_token = AccessToken::new(
                account_id,
                ps,
                Duration::seconds(config.auth.token_ttl.into()),
            );
            let encoded = SignedMessage::create(access_token.to_bytes(), private_key).encode();

            let thailand_offset = FixedOffset::east(7 * 60 * 60);
            let not_after = access_token
                .not_after()
                .with_timezone(&thailand_offset)
                .to_rfc3339_opts(SecondsFormat::Secs, true);
            Ok(SignInResponse {
                id: account_id,
                token: encoded,
                not_after,
            })
        } else {
            Err(UserFacingError::InvalidUserNameOrPassword.into())
        }
    } else {
        let duration = get_junk_hash_duration(config.auth.bcrypt_cost).await;
        tokio::time::sleep(duration).await;
        Err(UserFacingError::InvalidUserNameOrPassword.into())
    }
}
