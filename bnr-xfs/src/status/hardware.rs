use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Represents the overall status of the CDR hardware.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HardwareStatus {
    #[default]
    Ok = 0,
    Missing,
    Notification,
    Warning,
    Error,
}

impl HardwareStatus {
    /// Creates a new [HardwareStatus].
    pub const fn new() -> Self {
        Self::Ok
    }

    /// Gets the [XfsMember] name for [HardwareStatus].
    pub const fn xfs_name() -> &'static str {
        "hardwareStatus"
    }
}

impl From<u32> for HardwareStatus {
    fn from(val: u32) -> Self {
        match val {
            hs if hs == 0 => Self::Ok,
            hs if hs == 1 => Self::Missing,
            hs if hs == 2 => Self::Notification,
            hs if hs == 3 => Self::Warning,
            hs if hs == 4 => Self::Error,
            _ => Self::Missing,
        }
    }
}

impl From<&u32> for HardwareStatus {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<HardwareStatus> for u32 {
    fn from(val: HardwareStatus) -> Self {
        val as u32
    }
}

impl From<&HardwareStatus> for u32 {
    fn from(val: &HardwareStatus) -> Self {
        (*val).into()
    }
}

impl From<i32> for HardwareStatus {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for HardwareStatus {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<HardwareStatus> for i32 {
    fn from(val: HardwareStatus) -> Self {
        val as i32
    }
}

impl From<&HardwareStatus> for i32 {
    fn from(val: &HardwareStatus) -> Self {
        (*val).into()
    }
}

impl From<&[HardwareStatus]> for HardwareStatus {
    fn from(val: &[HardwareStatus]) -> Self {
        // start by checking for the "highest" priority status, i.e. status that require the most
        // attention
        if val.contains(&Self::Error) {
            Self::Error
        } else if val.contains(&Self::Warning) {
            Self::Warning
        } else if val.contains(&Self::Missing) {
            Self::Missing
        } else if val.contains(&Self::Notification) {
            Self::Notification
        } else {
            Self::Ok
        }
    }
}

impl<const N: usize> From<&[HardwareStatus; N]> for HardwareStatus {
    fn from(val: &[HardwareStatus; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<[HardwareStatus; N]> for HardwareStatus {
    fn from(val: [HardwareStatus; N]) -> Self {
        val.as_ref().into()
    }
}

impl From<HardwareStatus> for &'static str {
    fn from(val: HardwareStatus) -> Self {
        match val {
            HardwareStatus::Ok => "OK",
            HardwareStatus::Missing => "missing",
            HardwareStatus::Notification => "notification",
            HardwareStatus::Warning => "warning",
            HardwareStatus::Error => "error",
        }
    }
}

impl From<&HardwareStatus> for &'static str {
    fn from(val: &HardwareStatus) -> Self {
        (*val).into()
    }
}

impl From<&HardwareStatus> for XfsValue {
    fn from(val: &HardwareStatus) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<HardwareStatus> for XfsValue {
    fn from(val: HardwareStatus) -> Self {
        (&val).into()
    }
}

impl From<&HardwareStatus> for XfsMember {
    fn from(val: &HardwareStatus) -> Self {
        XfsMember::create(HardwareStatus::xfs_name(), val.into())
    }
}

impl From<HardwareStatus> for XfsMember {
    fn from(val: HardwareStatus) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for HardwareStatus {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0i32).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected HardwareStatus XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for HardwareStatus {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for HardwareStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
