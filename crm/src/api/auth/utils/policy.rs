use std::fmt::{self, Debug, Display};

use tracing::instrument;

use tokidator::rbac::{json_discriminant_array_to_vec, PolicySet, RoleSet};

use crate::auth::rbac::{Policy, Role};

#[derive(Debug)]
enum ErrorKind {
    InvalidRoleId(String),
    InvalidPolicyId(String),
}

#[derive(Debug)]
pub struct JoinArrayToPolicySetError {
    account_id: i64,
    kind: ErrorKind,
}

impl Display for JoinArrayToPolicySetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self.kind {
            InvalidRoleId(ref id) => write!(
                f,
                "Account id = {} has invalid role id = {}",
                self.account_id, id
            ),
            InvalidPolicyId(ref id) => write!(
                f,
                "Account id = {} has invalid policy id = {}",
                self.account_id, id
            ),
        }
    }
}

impl std::error::Error for JoinArrayToPolicySetError {}

#[instrument(err, skip(unparsed_roles, unparsed_policies))]
pub fn json_array_to_policy_set(
    account_id: i64,
    unparsed_roles: &str,
    unparsed_policies: &str,
) -> Result<PolicySet<Policy>, JoinArrayToPolicySetError> {
    let roles: RoleSet<Role> = json_discriminant_array_to_vec(unparsed_roles)
        .map_err(|s| JoinArrayToPolicySetError {
            account_id,
            kind: ErrorKind::InvalidRoleId(s.to_string()),
        })?
        .into();
    let mut policy_set = roles.to_policy_set();
    let policies: Vec<Policy> = json_discriminant_array_to_vec(unparsed_policies).map_err(|s| {
        JoinArrayToPolicySetError {
            account_id,
            kind: ErrorKind::InvalidPolicyId(s.to_string()),
        }
    })?;
    policy_set.extend(policies);
    Ok(policy_set)
}
