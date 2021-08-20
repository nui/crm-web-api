use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::rbac::traits::Role;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::Display,
    strum::EnumCount,
)]
pub enum TestPolicy {
    Policy0,
    Policy1,
    Policy2,
    Policy3,
    Policy4,
    Policy5,
    Policy6,
    Policy7,
    Policy8,
    Policy9,
    Policy10,
    Policy11,
    Policy12,
    Policy13,
    Policy14,
    Policy15,
}

impl crate::rbac::Policy for TestPolicy {}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::Display,
)]
pub enum TestRole {
    Role0,
    Role1,
    Role2,
}

impl Role for TestRole {
    type Policy = TestPolicy;

    fn policies(&self) -> Vec<Self::Policy> {
        POLICIES.get(self).cloned().unwrap_or_default()
    }
}

type RolePoliciesMap = BTreeMap<TestRole, Vec<TestPolicy>>;

static POLICIES: Lazy<RolePoliciesMap> = Lazy::new(create_role_policies);

fn create_role_policies() -> RolePoliciesMap {
    use TestPolicy::*;
    use TestRole::*;
    let mut map = RolePoliciesMap::new();
    map.insert(Role0, vec![Policy0, Policy1]);
    map.insert(Role2, vec![Policy3, Policy4]);
    map
}
