use std::fmt;

use crate::xfs::{value::XfsValue, xfs_struct::XfsMember};
use crate::{Error, Result};

/// Defines how the BNR perform the self tests.
///
/// Integration recommendations:
///
/// If the device mode [Device](SelfTestMode::Device) is used, the Host should send a [self_test](crate::maintenance::self_test) command at every time that internal tests and movements of the BNR can be allowed.
///
/// It is recommended to send [self_test](crate::maintenance::self_test), after a [present](crate::cash::present) command to allow the BNR to refloat the recyclers.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SelfTestMode {
    /// This is the default and recommended mode. In this mode, the BNR does self tests automatically.
    /// After [reset](crate::init::reset) or [present](crate::cash::present), the BNR refloats the Recyclers from the Loader, when necessary.
    #[default]
    Auto = 0,
    /// In this mode, the BNR doesn’t do anything automatically, but waits for a [self_test](crate::maintenance::self_test) command to do all self test actions.
    Device,
}

impl SelfTestMode {
    /// Creates a new [SelfTestMode].
    pub const fn new() -> Self {
        Self::Auto
    }

    /// Gets the [XfsMember] name for [SelfTestMode].
    pub const fn xfs_name() -> &'static str {
        "selfTestMode"
    }
}

impl From<u32> for SelfTestMode {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Auto,
            1 => Self::Device,
            _ => Self::Auto,
        }
    }
}

impl From<&u32> for SelfTestMode {
    fn from(val: &u32) -> Self {
        (*val).into()
    }
}

impl From<i32> for SelfTestMode {
    fn from(val: i32) -> Self {
        (val as u32).into()
    }
}

impl From<&i32> for SelfTestMode {
    fn from(val: &i32) -> Self {
        (*val).into()
    }
}

impl From<SelfTestMode> for u32 {
    fn from(val: SelfTestMode) -> Self {
        val as u32
    }
}

impl From<&SelfTestMode> for u32 {
    fn from(val: &SelfTestMode) -> Self {
        (*val).into()
    }
}

impl From<SelfTestMode> for i32 {
    fn from(val: SelfTestMode) -> Self {
        val as i32
    }
}

impl From<&SelfTestMode> for i32 {
    fn from(val: &SelfTestMode) -> Self {
        (*val).into()
    }
}

impl From<&SelfTestMode> for &'static str {
    fn from(val: &SelfTestMode) -> Self {
        match val {
            SelfTestMode::Auto => "auto",
            SelfTestMode::Device => "device",
        }
    }
}

impl From<SelfTestMode> for &'static str {
    fn from(val: SelfTestMode) -> Self {
        (&val).into()
    }
}

impl From<&SelfTestMode> for XfsValue {
    fn from(val: &SelfTestMode) -> Self {
        Self::new().with_i4(val.into())
    }
}

impl From<SelfTestMode> for XfsValue {
    fn from(val: SelfTestMode) -> Self {
        (&val).into()
    }
}

impl From<&SelfTestMode> for XfsMember {
    fn from(val: &SelfTestMode) -> Self {
        XfsMember::create(SelfTestMode::xfs_name(), val.into())
    }
}

impl From<SelfTestMode> for XfsMember {
    fn from(val: SelfTestMode) -> Self {
        (&val).into()
    }
}

impl TryFrom<&XfsMember> for SelfTestMode {
    type Error = Error;

    fn try_from(val: &XfsMember) -> Result<Self> {
        if val.name() == Self::xfs_name() && val.value().i4().is_some() {
            Ok(val.value().i4().unwrap_or(&0).into())
        } else {
            Err(Error::Xfs(format!(
                "Expected SelfTestMode XfsMember, have: {val}"
            )))
        }
    }
}

impl TryFrom<XfsMember> for SelfTestMode {
    type Error = Error;

    fn try_from(val: XfsMember) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for SelfTestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
