use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

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

    /// Gets the [XfsMember] name for [AntiFishingLevel].
    pub const fn xfs_name() -> &'static str {
        "antiFishingLevel"
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

impl From<&u32> for AntiFishingLevel {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for AntiFishingLevel {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for AntiFishingLevel {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<AntiFishingLevel> for u32 {
    fn from(val: AntiFishingLevel) -> Self {
        val as u32
    }
}

impl From<&AntiFishingLevel> for u32 {
    fn from(val: &AntiFishingLevel) -> Self {
        (*val).into()
    }
}

impl From<AntiFishingLevel> for i32 {
    fn from(val: AntiFishingLevel) -> Self {
        val as i32
    }
}

impl From<&AntiFishingLevel> for i32 {
    fn from(val: &AntiFishingLevel) -> Self {
        (*val).into()
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

impl From<&AntiFishingLevel> for XfsValue {
    fn from(val: &AntiFishingLevel) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<AntiFishingLevel> for XfsValue {
    fn from(val: AntiFishingLevel) -> Self {
        (&val).into()
    }
}

impl From<&AntiFishingLevel> for XfsMember {
    fn from(val: &AntiFishingLevel) -> Self {
        XfsMember::create(AntiFishingLevel::xfs_name(), val.into())
    }
}

impl From<AntiFishingLevel> for XfsMember {
    fn from(val: AntiFishingLevel) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for AntiFishingLevel {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected AntiFishingLevel XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for AntiFishingLevel {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for AntiFishingLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
