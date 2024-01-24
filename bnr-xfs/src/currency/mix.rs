use std::fmt;

use crate::impl_xfs_enum;

const ALGORITHM: u32 = 6041;
const TABLE: u32 = 6042;
const DENOM: u32 = 6043;
const MIN_BILLS: u32 = 6044;
const ALGORITHM_BASE: u32 = 6063;
const OPTIMUM_CHANGE: u32 = 6064;

/// Represents variants for mixing algorithms in different contexts of CDR operation.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MixNumber {
    #[default]
    Algorithm = ALGORITHM,
    Table = TABLE,
    Denom = DENOM,
    MinBills = MIN_BILLS,
    AlgorithmBase = ALGORITHM_BASE,
    OptimumChange = OPTIMUM_CHANGE,
}

impl MixNumber {
    /// Creates a new [MixNumber].
    pub const fn new() -> Self {
        Self::Algorithm
    }

    /// Creates a new [MixNumber] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            v if v == ALGORITHM => Self::Algorithm,
            v if v == TABLE => Self::Table,
            v if v == DENOM => Self::Denom,
            v if v == MIN_BILLS => Self::MinBills,
            v if v == ALGORITHM_BASE => Self::AlgorithmBase,
            v if v == OPTIMUM_CHANGE => Self::OptimumChange,
            _ => Self::Algorithm,
        }
    }
}

impl From<&MixNumber> for &'static str {
    fn from(val: &MixNumber) -> Self {
        match val {
            MixNumber::Algorithm => "algorithm",
            MixNumber::Table => "table",
            MixNumber::Denom => "denomination",
            MixNumber::MinBills => "minimum bills",
            MixNumber::AlgorithmBase => "algorithm base",
            MixNumber::OptimumChange => "optimum change",
        }
    }
}

impl From<MixNumber> for &'static str {
    fn from(val: MixNumber) -> Self {
        (&val).into()
    }
}

impl fmt::Display for MixNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(MixNumber, "mixNumber");
