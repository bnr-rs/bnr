use std::fmt;

pub const CDR_POS_TOP: u32 = 1;
pub const CDR_POS_BOTTOM: u32 = 2;

/// Represents a CDR position
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CdrPosition {
    /// Inlet position
    Top = CDR_POS_TOP,
    /// Outlet position
    #[default]
    Bottom = CDR_POS_BOTTOM,
}

impl CdrPosition {
    /// Creates a new [CdrPosition].
    pub const fn new() -> Self {
        Self::Bottom
    }
    /// Creates a new [CdrPosition] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ds if ds == CDR_POS_BOTTOM => Self::Bottom,
            ds if ds == CDR_POS_TOP => Self::Top,
            _ => Self::Bottom,
        }
    }
}

impl From<CdrPosition> for &'static str {
    fn from(val: CdrPosition) -> Self {
        match val {
            CdrPosition::Bottom => "bottom",
            CdrPosition::Top => "top",
        }
    }
}

impl From<&CdrPosition> for &'static str {
    fn from(val: &CdrPosition) -> Self {
        (*val).into()
    }
}

impl fmt::Display for CdrPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(CdrPosition, "position");
