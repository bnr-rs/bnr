use std::fmt;

use crate::impl_xfs_enum;

const ANTI_FISH_NORMAL: u32 = 0;
const ANTI_FISH_HIGH: u32 = 1;
const ANTI_FISH_SPECIAL: u32 = 2;

/// Defines the sensitivity level of string detection at Inlet.
///
/// See [Capabilities].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AntiFishingLevel {
    /// Normal sensitivity (default).
    #[default]
    Normal = ANTI_FISH_NORMAL,
    /// High sensitivity.
    High = ANTI_FISH_HIGH,
    /// Special sensitivity.
    ///
    /// Special mode for use in applications where rain water blows into the BNR bezel, please @ref CONTACT “contact” CPI technical support for details.
    Special = ANTI_FISH_SPECIAL,
}

impl AntiFishingLevel {
    /// Creates a new [AntiFishingLevel].
    pub const fn new() -> Self {
        Self::Normal
    }

    /// Creates a new [AntiFishingLevel] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ANTI_FISH_NORMAL => Self::Normal,
            ANTI_FISH_HIGH => Self::High,
            ANTI_FISH_SPECIAL => Self::Special,
            _ => Self::Normal,
        }
    }
}

impl From<&AntiFishingLevel> for &'static str {
    fn from(val: &AntiFishingLevel) -> Self {
        match val {
            AntiFishingLevel::Normal => "normal",
            AntiFishingLevel::High => "high",
            AntiFishingLevel::Special => "special",
        }
    }
}

impl From<AntiFishingLevel> for &'static str {
    fn from(val: AntiFishingLevel) -> Self {
        (&val).into()
    }
}

impl fmt::Display for AntiFishingLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(AntiFishingLevel, "antiFishingLevel");
