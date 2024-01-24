use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Values for indication of the security level in communication between Host and BNR.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SecuredCommLevel {
    /// Standard communication level.
    #[default]
    Level1 = 0,
    /// Secured communication level.
    Level2,
    /// Error occurred.
    Error,
}

impl SecuredCommLevel {
    /// Creates a new [SecuredCommLevel].
    pub const fn new() -> Self {
        Self::Level1
    }

    /// Gets the [XfsMember] name for [SecuredCommLevel].
    pub const fn xfs_name() -> &'static str {
        "securedCommLevel"
    }
}

impl From<u32> for SecuredCommLevel {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Level1,
            1 => Self::Level2,
            2 => Self::Error,
            _ => Self::Level1,
        }
    }
}

impl From<&u32> for SecuredCommLevel {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for SecuredCommLevel {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for SecuredCommLevel {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<SecuredCommLevel> for u32 {
    fn from(val: SecuredCommLevel) -> Self {
        val as u32
    }
}

impl From<&SecuredCommLevel> for u32 {
    fn from(val: &SecuredCommLevel) -> Self {
        (*val).into()
    }
}

impl From<SecuredCommLevel> for i32 {
    fn from(val: SecuredCommLevel) -> Self {
        val as i32
    }
}

impl From<&SecuredCommLevel> for i32 {
    fn from(val: &SecuredCommLevel) -> Self {
        (*val).into()
    }
}

impl From<&SecuredCommLevel> for &'static str {
    fn from(val: &SecuredCommLevel) -> Self {
        match val {
            SecuredCommLevel::Level1 => "level1",
            SecuredCommLevel::Level2 => "level2",
            SecuredCommLevel::Error => "error",
        }
    }
}

impl From<SecuredCommLevel> for &'static str {
    fn from(val: SecuredCommLevel) -> Self {
        (&val).into()
    }
}

impl From<&SecuredCommLevel> for XfsValue {
    fn from(val: &SecuredCommLevel) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<SecuredCommLevel> for XfsValue {
    fn from(val: SecuredCommLevel) -> Self {
        (&val).into()
    }
}

impl From<&SecuredCommLevel> for XfsMember {
    fn from(val: &SecuredCommLevel) -> Self {
        XfsMember::create(SecuredCommLevel::xfs_name(), val.into())
    }
}

impl From<SecuredCommLevel> for XfsMember {
    fn from(val: SecuredCommLevel) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for SecuredCommLevel {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected SecuredCommLevel XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for SecuredCommLevel {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for SecuredCommLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
