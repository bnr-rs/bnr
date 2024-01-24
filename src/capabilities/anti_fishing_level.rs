use std::fmt;

/// Defines the sensitivity level of string detection at Inlet.
///
/// See [Capabilities].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AntiFishingLevel {
    /// Normal sensitivity (default).
    #[default]
    Normal = 0,
    /// High sensitivity.
    High,
    /// Special sensitivity.
    ///
    /// Special mode for use in applications where rain water blows into the BNR bezel, please @ref CONTACT “contact” CPI technical support for details.
    Special,
}

impl AntiFishingLevel {
    /// Creates a new [AntiFishingLevel].
    pub const fn new() -> Self {
        Self::Normal
    }
}

impl From<u32> for AntiFishingLevel {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::High,
            2 => Self::Special,
            _ => Self::Normal,
        }
    }
}

impl From<AntiFishingLevel> for u32 {
    fn from(val: AntiFishingLevel) -> Self {
        val as u32
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
