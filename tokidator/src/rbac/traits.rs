use num_traits::{FromPrimitive, ToPrimitive};

/// A marker trait for enums where variants do not have payloads
///
/// ToPrimitive must produce unique value (same value that use in Ord)
pub trait Policy: Copy + Clone + Ord + FromPrimitive + ToPrimitive {}

pub trait Role: Ord + FromPrimitive {
    type Policy: Policy;

    fn policies(&self) -> Vec<Self::Policy>;
}
