use std::fmt;

use super::HardwareStatus;

pub const SD_OPEN: u32 = 6194;
pub const SD_LOCKED: u32 = 6196;
pub const SD_UNKNOWN: u32 = 6197;

/// Represents safe door status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SafeDoorStatus {
    /// Indicates the safe door is open
    Open = SD_OPEN,
    /// Indicates the safe door is locked
    Locked = SD_LOCKED,
    /// Indicates the safe door status is unknown
    #[default]
    Unknown = SD_UNKNOWN,
}

impl SafeDoorStatus {
    /// Creates a new [SafeDoorStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [SafeDoorStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ds if ds == SD_OPEN => Self::Open,
            ds if ds == SD_LOCKED => Self::Locked,
            ds if ds == SD_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<SafeDoorStatus> for HardwareStatus {
    fn from(val: SafeDoorStatus) -> Self {
        match val {
            SafeDoorStatus::Locked | SafeDoorStatus::Open => Self::Notification,
            SafeDoorStatus::Unknown => Self::Warning,
        }
    }
}

impl From<SafeDoorStatus> for &'static str {
    fn from(val: SafeDoorStatus) -> Self {
        match val {
            SafeDoorStatus::Open => "open",
            SafeDoorStatus::Locked => "locked",
            SafeDoorStatus::Unknown => "unknown",
        }
    }
}

impl From<&SafeDoorStatus> for &'static str {
    fn from(val: &SafeDoorStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for SafeDoorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_xfs_enum!(SafeDoorStatus, "safeDoorStatus");
