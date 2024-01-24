use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Defines the kind of error report to be generated when a failure is detected whith no bill being transported.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ReportingMode {
    /// Normal reporting mode (default): on failure detection, a SimpleFailureReport will saved.
    #[default]
    Normal = 0,
    /// Full reporting mode: on failure detection, a BillTransportErrorReport will saved.
    Full,
}

impl ReportingMode {
    /// Creates a new [ReportingMode].
    pub const fn new() -> Self {
        Self::Normal
    }

    /// Gets the [XfsMember] name for [ReportingMode].
    pub const fn xfs_name() -> &'static str {
        "reportingMode"
    }
}

impl From<u32> for ReportingMode {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::Full,
            _ => Self::Normal,
        }
    }
}

impl From<&u32> for ReportingMode {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for ReportingMode {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for ReportingMode {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<ReportingMode> for u32 {
    fn from(val: ReportingMode) -> Self {
        val as u32
    }
}

impl From<&ReportingMode> for u32 {
    fn from(val: &ReportingMode) -> Self {
        (*val).into()
    }
}

impl From<ReportingMode> for i32 {
    fn from(val: ReportingMode) -> Self {
        val as i32
    }
}

impl From<&ReportingMode> for i32 {
    fn from(val: &ReportingMode) -> Self {
        (*val).into()
    }
}

impl From<&ReportingMode> for &'static str {
    fn from(val: &ReportingMode) -> Self {
        match val {
            ReportingMode::Normal => "normal",
            ReportingMode::Full => "full",
        }
    }
}

impl From<ReportingMode> for &'static str {
    fn from(val: ReportingMode) -> Self {
        (&val).into()
    }
}

impl From<&ReportingMode> for XfsValue {
    fn from(val: &ReportingMode) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<ReportingMode> for XfsValue {
    fn from(val: ReportingMode) -> Self {
        (&val).into()
    }
}

impl From<&ReportingMode> for XfsMember {
    fn from(val: &ReportingMode) -> Self {
        XfsMember::create(ReportingMode::xfs_name(), val.into())
    }
}

impl From<ReportingMode> for XfsMember {
    fn from(val: ReportingMode) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for ReportingMode {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected ReportingMode XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for ReportingMode {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for ReportingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
