use protobuf::Message;

use crate::rbac::test_helpers::TestPolicy;
use crate::rbac::PolicySet;

use super::PolicyAccessToken;

#[derive(Debug)]
pub struct TestAccessToken {
    policies: PolicySet<TestPolicy>,
    expired: bool,
}

impl TestAccessToken {
    pub fn new(policies: PolicySet<TestPolicy>, expired: bool) -> TestAccessToken {
        Self { policies, expired }
    }
}

impl PolicyAccessToken for TestAccessToken {
    type Policy = TestPolicy;
    type Error = ();

    fn policies(&self) -> &PolicySet<Self::Policy> {
        &self.policies
    }

    fn is_expired(&self) -> bool {
        self.expired
    }

    fn to_bytes(&self) -> Vec<u8> {
        let policies = self.policies.to_bytes();
        let mut builder = crate::protos::TestAccessToken::new();
        builder.set_policies(policies);
        builder.set_expired(self.expired);
        builder
            .write_to_bytes()
            .expect("Fail build bytes from test policy")
    }

    fn from_bytes(buf: &[u8]) -> Result<Self, Self::Error> {
        let token = crate::protos::TestAccessToken::parse_from_bytes(buf).map_err(drop)?;
        let ps = PolicySet::parse_from_bytes(token.policies.as_slice())
            .expect("Bad encoded test policies");
        Ok(Self::new(ps, token.expired))
    }
}
