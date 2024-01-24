/// Represents variants for mixing algorithms in different contexts of CDR operation.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MixNumber {
    #[default]
    Algorithm = 6041,
    Table = 6042,
    Denom = 6043,
    MinBills = 6044,
    AlgorithmBase = 6063,
    OptimumChange = 6064,
}

impl MixNumber {
    /// Creates a new [MixNumber].
    pub const fn new() -> Self {
        Self::Algorithm
    }
}

impl From<u32> for MixNumber {
    fn from(val: u32) -> Self {
        match val {
            v if v == 6041 => Self::Algorithm,
            v if v == 6042 => Self::Table,
            v if v == 6043 => Self::Denom,
            v if v == 6044 => Self::MinBills,
            v if v == 6063 => Self::AlgorithmBase,
            v if v == 6064 => Self::OptimumChange,
            _ => Self::Algorithm,
        }
    }
}

impl From<MixNumber> for u32 {
    fn from(val: MixNumber) -> Self {
        val as u32
    }
}
