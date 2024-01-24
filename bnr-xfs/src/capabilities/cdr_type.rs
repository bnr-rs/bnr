use std::fmt;

use crate::impl_xfs_enum;

const CDR_TYPE_NONE: u32 = 6010;
const CDR_TYPE_DISPENSER: u32 = 6011;
const CDR_TYPE_RECYCLER: u32 = 6012;
const CDR_TYPE_ATM: u32 = 6013;

/// Types of CDR units.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CdrType {
    #[default]
    None = CDR_TYPE_NONE,
    Dispenser = CDR_TYPE_DISPENSER,
    Recycler = CDR_TYPE_RECYCLER,
    Atm = CDR_TYPE_ATM,
}

impl CdrType {
    /// Creates a new [CdrType].
    pub const fn new() -> Self {
        Self::None
    }

    /// Creates a new [CdrType] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            CDR_TYPE_NONE => Self::None,
            CDR_TYPE_DISPENSER => Self::Dispenser,
            CDR_TYPE_RECYCLER => Self::Recycler,
            CDR_TYPE_ATM => Self::Atm,
            _ => Self::None,
        }
    }
}

impl From<&CdrType> for &'static str {
    fn from(val: &CdrType) -> Self {
        match val {
            CdrType::None => "none",
            CdrType::Dispenser => "dispenser",
            CdrType::Recycler => "recycler",
            CdrType::Atm => "atm",
        }
    }
}

impl From<CdrType> for &'static str {
    fn from(val: CdrType) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CdrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(CdrType, "cdType");
