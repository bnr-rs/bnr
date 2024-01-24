use std::fmt;

use super::HardwareStatus;

/// Represents safe door status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SafeDoorStatus {
    /// Indicates the safe door is locked
    Locked = bnr_sys::XFS_S_CDR_SD_LOCKED,
    /// Indicates the safe door is open
    Open = bnr_sys::XFS_S_CDR_SD_OPEN,
    /// Indicates the safe door status is unknown
    #[default]
    Unknown = bnr_sys::XFS_S_CDR_SD_UNKNOWN,
}

impl SafeDoorStatus {
    /// Creates a new [SafeDoorStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for SafeDoorStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XFS_S_CDR_SD_LOCKED => Self::Locked,
            ds if ds == bnr_sys::XFS_S_CDR_SD_OPEN => Self::Open,
            ds if ds == bnr_sys::XFS_S_CDR_SD_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<SafeDoorStatus> for u32 {
    fn from(val: SafeDoorStatus) -> Self {
        val as u32
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
            SafeDoorStatus::Locked => "locked",
            SafeDoorStatus::Open => "open",
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
