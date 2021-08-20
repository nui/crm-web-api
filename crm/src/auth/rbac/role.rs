use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::auth::rbac::Policy;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    strum::Display,
    strum::EnumCount,
)]
#[repr(u16)]
pub enum Role {
    _Unused_,
    Admin,
    MakerUser,
}

impl tokidator::rbac::Role for Role {
    type Policy = Policy;

    fn policies(&self) -> Vec<Self::Policy> {
        POLICIES.get(self).cloned().unwrap_or_default()
    }
}

type RolePoliciesMap = BTreeMap<Role, Vec<Policy>>;

static POLICIES: Lazy<RolePoliciesMap> = Lazy::new(create_role_policies);

fn create_role_policies() -> RolePoliciesMap {
    use Policy::*;
    use Role::*;
    let mut map = RolePoliciesMap::new();
    map.insert(
        Admin,
        vec![
            CreateAccount,
            EncodePassword,
            ListAccounts,
            ListAnyActor,
            ParseAccessToken,
            QueryGraphQL,
            _Reserve16_,
        ],
    );
    map
}
