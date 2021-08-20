use std::iter::FromIterator;

use PolicyCondition::*;

use crate::rbac::Policy;
use crate::rbac::PolicySet;

#[derive(Clone)]
pub enum PolicyCondition<P: Policy> {
    Nil,
    Contains(P),
    Any(Box<[P]>),
    All(Box<[P]>),
}

impl<P: Policy> PolicyCondition<P> {
    pub fn satisfy(&self, policies: &PolicySet<P>) -> bool {
        let set = policies.inner();
        match self {
            Nil => true,
            Contains(policy) => set.contains(policy),
            Any(slice) => slice.iter().any(|policy| set.contains(policy)),
            All(slice) => slice.iter().all(|policy| set.contains(policy)),
        }
    }

    pub fn contains(policy: P) -> Self {
        Contains(policy)
    }

    pub fn all<T: IntoIterator<Item = P>>(iter: T) -> Self {
        All(Vec::from_iter(iter).into_boxed_slice())
    }

    pub fn any<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Any(Vec::from_iter(iter).into_boxed_slice())
    }
}

impl<P: Policy> From<P> for PolicyCondition<P> {
    fn from(p: P) -> Self {
        Contains(p)
    }
}

impl<P: Policy> AsRef<PolicyCondition<P>> for PolicyCondition<P> {
    fn as_ref(&self) -> &PolicyCondition<P> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPolicy::{self, *};

    use super::*;

    type Condition = PolicyCondition<TestPolicy>;

    #[test]
    fn no_should_always_satisfy() {
        assert!(Condition::Nil.satisfy(&vec![].into()));
        assert!(Condition::Nil.satisfy(&vec![Policy1].into()));
        assert!(Condition::Nil.satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_satisfy() {
        assert!(Condition::contains(Policy1).satisfy(&vec![Policy1].into()));
        assert!(Condition::contains(Policy1).satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_not_satisfy() {
        assert!(!Condition::contains(Policy1).satisfy(&vec![].into()));
        assert!(!Condition::contains(Policy1).satisfy(&vec![Policy2].into()));
    }

    #[test]
    fn any_should_satisfy() {
        let check_ps: PolicySet<_> = vec![Policy1, Policy2].into();
        assert!(Condition::any(vec![Policy1]).satisfy(&check_ps));
        assert!(Condition::any(vec![Policy2]).satisfy(&check_ps));
        assert!(Condition::any(vec![Policy1, Policy2]).satisfy(&check_ps));
        assert!(Condition::any(vec![Policy1, Policy2, Policy3]).satisfy(&check_ps));
    }

    #[test]
    fn any_should_not_satisfy() {
        assert!(!Condition::any(vec![]).satisfy(&vec![].into()));
        assert!(!Condition::any(vec![]).satisfy(&vec![Policy1].into()));
        assert!(!Condition::any(vec![]).satisfy(&vec![Policy1, Policy2].into()));

        assert!(!Condition::any(vec![Policy1]).satisfy(&vec![].into()));
        assert!(!Condition::any(vec![Policy1, Policy2]).satisfy(&vec![].into()));

        assert!(!Condition::any(vec![Policy1]).satisfy(&vec![Policy2].into()));
        assert!(!Condition::any(vec![Policy1]).satisfy(&vec![Policy2, Policy3].into()));
    }

    #[test]
    fn all_should_satisfy() {
        assert!(Condition::all(vec![Policy1]).satisfy(&vec![Policy1].into()));
        assert!(Condition::all(vec![Policy1, Policy2]).satisfy(&vec![Policy1, Policy2].into()));
        assert!(
            Condition::all(vec![Policy1, Policy2]).satisfy(&vec![Policy1, Policy2, Policy3].into())
        );

        assert!(Condition::all(vec![]).satisfy(&vec![].into()));
        assert!(Condition::all(vec![]).satisfy(&vec![Policy1].into()));
        assert!(Condition::all(vec![]).satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn all_should_not_satisfy() {
        assert!(!Condition::all(vec![Policy1]).satisfy(&vec![].into()));
        assert!(!Condition::all(vec![Policy1, Policy2]).satisfy(&vec![].into()));

        assert!(!Condition::all(vec![Policy1, Policy2]).satisfy(&vec![Policy1].into()));
        assert!(!Condition::all(vec![Policy1, Policy2]).satisfy(&vec![Policy2].into()));
        assert!(!Condition::all(vec![Policy1, Policy2]).satisfy(&vec![Policy2, Policy3].into()));
    }
}
