use std::fmt;

use super::HardwareStatus;

pub const SHT_CLOSED: u32 = 6198;
pub const SHT_OPEN: u32 = 6199;
pub const SHT_NOT_SUPPORTED: u32 = 6201;
pub const SHT_UNKNOWN: u32 = 6202;

/// Represents shutter status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ShutterStatus {
    /// Indicates the shutter is closed
    Closed = SHT_CLOSED,
    /// Indicates the shutter status is not supported
    NotSupported = SHT_NOT_SUPPORTED,
    /// Indicates the shutter is open
    Open = SHT_OPEN,
    /// Indicates the shutter status is unknown
    #[default]
    Unknown = SHT_UNKNOWN,
}

impl ShutterStatus {
    /// Creates a new [ShutterStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [ShutterStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ds if ds == SHT_CLOSED => Self::Closed,
            ds if ds == SHT_NOT_SUPPORTED => Self::NotSupported,
            ds if ds == SHT_OPEN => Self::Open,
            ds if ds == SHT_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<ShutterStatus> for HardwareStatus {
    fn from(val: ShutterStatus) -> Self {
        match val {
            ShutterStatus::Closed | ShutterStatus::Open => Self::Notification,
            ShutterStatus::NotSupported => Self::Missing,
            ShutterStatus::Unknown => Self::Warning,
        }
    }
}

impl From<ShutterStatus> for &'static str {
    fn from(val: ShutterStatus) -> Self {
        match val {
            ShutterStatus::Closed => "closed",
            ShutterStatus::Open => "open",
            ShutterStatus::NotSupported => "not supported",
            ShutterStatus::Unknown => "unknown",
        }
    }
}

impl From<&ShutterStatus> for &'static str {
    fn from(val: &ShutterStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ShutterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_xfs_enum!(ShutterStatus, "shutterStatus");
