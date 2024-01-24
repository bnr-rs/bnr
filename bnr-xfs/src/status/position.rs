use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

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

    /// Gets the [XfsMember] name of the [CdrPosition].
    pub const fn xfs_name() -> &'static str {
        "position"
    }
}

impl From<u32> for CdrPosition {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == CDR_POS_BOTTOM => Self::Bottom,
            ds if ds == CDR_POS_TOP => Self::Top,
            _ => Self::Bottom,
        }
    }
}

impl From<&u32> for CdrPosition {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for CdrPosition {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for CdrPosition {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<CdrPosition> for u32 {
    fn from(val: CdrPosition) -> Self {
        val as u32
    }
}

impl From<&CdrPosition> for u32 {
    fn from(val: &CdrPosition) -> Self {
        (*val).into()
    }
}

impl From<CdrPosition> for i32 {
    fn from(val: CdrPosition) -> Self {
        val as i32
    }
}

impl From<&CdrPosition> for i32 {
    fn from(val: &CdrPosition) -> Self {
        (*val).into()
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

impl From<&CdrPosition> for XfsValue {
    fn from(val: &CdrPosition) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<CdrPosition> for XfsValue {
    fn from(val: CdrPosition) -> Self {
        (&val).into()
    }
}

impl From<&CdrPosition> for XfsMember {
    fn from(val: &CdrPosition) -> Self {
        XfsMember::create(CdrPosition::xfs_name(), val.into())
    }
}

impl From<CdrPosition> for XfsMember {
    fn from(val: CdrPosition) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for CdrPosition {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected CdrPosition XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for CdrPosition {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CdrPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
