use std::ops::Add;

use chrono::prelude::*;
use chrono::Duration;
use protobuf::Message;
use serde_json::Value;
use tracing::trace;

use tokidator::rbac::PolicySet;
use tokidator::token::PolicyAccessToken;

use crate::auth::rbac::Policy;
use crate::protos::AccessTokenMsg;

pub struct AccessToken {
    pub account_id: i64,
    last_login: DateTime<Utc>,
    not_before: DateTime<Utc>,
    not_after: DateTime<Utc>,
    policies: PolicySet<Policy>,
}

impl AccessToken {
    pub fn new(account_id: i64, policies: PolicySet<Policy>, duration: Duration) -> Self {
        let now = Utc::now();
        Self {
            account_id,
            last_login: now,
            not_before: now,
            not_after: now.add(duration),
            policies,
        }
    }

    pub fn refresh(&self, policies: PolicySet<Policy>, duration: Duration) -> Self {
        let now = Utc::now();
        let Self {
            account_id,
            last_login,
            ..
        } = *self;
        Self {
            account_id,
            last_login,
            not_before: now,
            not_after: now.add(duration),
            policies,
        }
    }

    pub fn last_login(&self) -> &DateTime<Utc> {
        &self.last_login
    }

    pub fn not_after(&self) -> &DateTime<Utc> {
        &self.not_after
    }

    pub fn to_json(&self) -> Value {
        let mut policies: Vec<&str> = self.policies.iter().map(AsRef::as_ref).collect();
        policies.sort_unstable();
        serde_json::json!({
            "account_id": self.account_id,
            "last_login": self.last_login.to_rfc3339(),
            "not_before": self.not_before.to_rfc3339(),
            "not_after": self.not_after.to_rfc3339(),
            "policies": policies,
        })
    }
}

#[derive(Debug)]
pub enum ParseAccessTokenError {
    ProtobufError,
    UnknownPolicy,
}

impl PolicyAccessToken for AccessToken {
    type Policy = Policy;
    type Error = ParseAccessTokenError;

    fn policies(&self) -> &PolicySet<Self::Policy> {
        &self.policies
    }

    fn is_expired(&self) -> bool {
        Utc::now() >= self.not_after
    }

    fn to_bytes(&self) -> Vec<u8> {
        let policies = self.policies.to_bytes();
        trace!("encoded policies: {}", base64::encode(&policies));
        let mut builder = AccessTokenMsg::new();
        builder.set_accountId(self.account_id);
        builder.set_lastLogin(self.last_login.timestamp_millis());
        builder.set_notBefore(self.not_before.timestamp_millis());
        builder.set_notAfter(self.not_after.timestamp_millis());
        builder.set_policies(policies);
        builder
            .write_to_bytes()
            .expect("Fail to serialize access token")
    }

    fn from_bytes(buf: &[u8]) -> Result<Self, Self::Error> {
        let token =
            AccessTokenMsg::parse_from_bytes(buf).map_err(|_| Self::Error::ProtobufError)?;

        let policies =
            PolicySet::parse_from_bytes(&token.policies).map_err(|_| Self::Error::UnknownPolicy)?;

        let last_login = Utc.timestamp_millis(token.lastLogin);
        let not_before = Utc.timestamp_millis(token.notBefore);
        let not_after = Utc.timestamp_millis(token.notAfter);
        Ok(Self {
            account_id: token.accountId,
            last_login,
            not_before,
            not_after,
            policies,
        })
    }
}
