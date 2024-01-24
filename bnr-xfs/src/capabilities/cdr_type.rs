use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Types of CDR units.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CdrType {
    #[default]
    None = 6010,
    Dispenser = 6011,
    Recycler = 6012,
    Atm = 6013,
}

impl CdrType {
    /// Creates a new [CdrType].
    pub const fn new() -> Self {
        Self::None
    }

    /// Gets the [XfsMember] name for [CdrType].
    pub const fn xfs_name() -> &'static str {
        "cdType"
    }
}

impl From<u32> for CdrType {
    fn from(val: u32) -> Self {
        match val {
            6010 => Self::None,
            6011 => Self::Dispenser,
            6012 => Self::Recycler,
            6013 => Self::Atm,
            _ => Self::None,
        }
    }
}

impl From<&u32> for CdrType {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for CdrType {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for CdrType {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<CdrType> for u32 {
    fn from(val: CdrType) -> Self {
        val as u32
    }
}

impl From<&CdrType> for u32 {
    fn from(val: &CdrType) -> Self {
        (*val).into()
    }
}

impl From<CdrType> for i32 {
    fn from(val: CdrType) -> Self {
        val as i32
    }
}

impl From<&CdrType> for i32 {
    fn from(val: &CdrType) -> Self {
        (*val).into()
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

impl From<&CdrType> for XfsValue {
    fn from(val: &CdrType) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<CdrType> for XfsValue {
    fn from(val: CdrType) -> Self {
        (&val).into()
    }
}

impl From<&CdrType> for XfsMember {
    fn from(val: &CdrType) -> Self {
        XfsMember::create(CdrType::xfs_name(), val.into())
    }
}

impl From<CdrType> for XfsMember {
    fn from(val: CdrType) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for CdrType {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected CdrType XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for CdrType {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for CdrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
