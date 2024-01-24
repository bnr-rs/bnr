use std::fmt;

use super::HardwareStatus;

/// Represents CDR device status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DeviceStatus {
    /// Indicates a hardware error occured
    HardwareError = bnr_sys::XFS_S_CDR_DS_HARDWARE_ERROR,
    /// Device is offline
    Offline = bnr_sys::XFS_S_CDR_DS_OFF_LINE,
    #[default]
    /// Device is online
    Online = bnr_sys::XFS_S_CDR_DS_ON_LINE,
    /// Indicates a user error occured
    UserError = bnr_sys::XFS_S_CDR_DS_USER_ERROR,
    /// Indicates a change in the device status
    Changed = bnr_sys::XFS_S_CDR_DEVICE_STATUS_CHANGED,
}

impl DeviceStatus {
    /// Creates a new [DeviceStatus].
    pub const fn new() -> Self {
        Self::Online
    }
}

impl From<u32> for DeviceStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XFS_S_CDR_DS_HARDWARE_ERROR => Self::HardwareError,
            ds if ds == bnr_sys::XFS_S_CDR_DS_OFF_LINE => Self::Offline,
            ds if ds == bnr_sys::XFS_S_CDR_DS_ON_LINE => Self::Online,
            ds if ds == bnr_sys::XFS_S_CDR_DS_USER_ERROR => Self::UserError,
            ds if ds == bnr_sys::XFS_S_CDR_DEVICE_STATUS_CHANGED => Self::Changed,
            _ => Self::HardwareError,
        }
    }
}

impl From<DeviceStatus> for u32 {
    fn from(val: DeviceStatus) -> Self {
        val as u32
    }
}

impl From<DeviceStatus> for HardwareStatus {
    fn from(val: DeviceStatus) -> Self {
        match val {
            DeviceStatus::Online => Self::Ok,
            DeviceStatus::Changed => Self::Notification,
            DeviceStatus::Offline => Self::Missing,
            DeviceStatus::HardwareError | DeviceStatus::UserError => Self::Error,
        }
    }
}

impl From<DeviceStatus> for &'static str {
    fn from(val: DeviceStatus) -> Self {
        match val {
            DeviceStatus::HardwareError => "hardware error",
            DeviceStatus::Offline => "offline",
            DeviceStatus::Online => "online",
            DeviceStatus::UserError => "user error",
            DeviceStatus::Changed => "changed",
        }
    }
}

impl From<&DeviceStatus> for &'static str {
    fn from(val: &DeviceStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
