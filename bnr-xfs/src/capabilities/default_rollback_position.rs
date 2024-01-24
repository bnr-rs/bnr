use std::fmt;

use crate::{
    impl_xfs_enum,
    status::{CDR_POS_BOTTOM, CDR_POS_TOP},
};

/// Represents a CDR position
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DefaultRollbackPosition {
    /// Inlet position
    Top = CDR_POS_TOP,
    /// Outlet position
    #[default]
    Bottom = CDR_POS_BOTTOM,
}

impl DefaultRollbackPosition {
    /// Creates a new [DefaultRollbackPosition].
    pub const fn new() -> Self {
        Self::Bottom
    }

    /// Creates a new [DefaultRollbackPosition] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            CDR_POS_BOTTOM => Self::Bottom,
            CDR_POS_TOP => Self::Top,
            _ => Self::Bottom,
        }
    }
}

impl From<&DefaultRollbackPosition> for &'static str {
    fn from(val: &DefaultRollbackPosition) -> Self {
        match val {
            DefaultRollbackPosition::Bottom => "bottom",
            DefaultRollbackPosition::Top => "top",
        }
    }
}

impl From<DefaultRollbackPosition> for &'static str {
    fn from(val: DefaultRollbackPosition) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DefaultRollbackPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(DefaultRollbackPosition, "defaultRollbackPosition");
