#[allow(clippy::upper_case_acronyms)]
#[derive(
    Clone,
    Copy,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
)]
#[repr(u16)]
pub enum Policy {
    _Unused_,
    _Reserve1_,
    _Reserve2_,
    _Reserve3_,
    _Reserve4_,
    _Reserve5_,
    _Reserve6_,
    _Reserve7_,
    _Reserve8_,
    _Reserve9_,
    ParseAccessToken,
    EncodePassword,
    QueryGraphQL,
    ListAnyActor,
    ListAccounts,
    CreateAccount,
    // This is added to help manual test PolicySet::to_bytes
    _Reserve16_,
}

impl tokidator::rbac::Policy for Policy {}
