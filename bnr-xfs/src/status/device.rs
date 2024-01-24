use std::fmt;

use super::HardwareStatus;

pub const DEVICE_STATUS_CHANGED: u32 = 6162;
pub const HARDWARE_ERROR: u32 = 6174;
pub const USER_ERROR: u32 = 6175;
pub const OFF_LINE: u32 = 6179;
pub const ON_LINE: u32 = 6180;

/// Represents CDR device status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DeviceStatus {
    /// Indicates a change in the device status
    Changed = DEVICE_STATUS_CHANGED,
    /// Indicates a hardware error occured
    HardwareError = HARDWARE_ERROR,
    /// Indicates a user error occured
    UserError = USER_ERROR,
    /// Device is offline
    Offline = OFF_LINE,
    #[default]
    /// Device is online
    Online = ON_LINE,
}

impl DeviceStatus {
    /// Creates a new [DeviceStatus].
    pub const fn new() -> Self {
        Self::Online
    }

    /// Creates a new [DeviceStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            DEVICE_STATUS_CHANGED => Self::Changed,
            HARDWARE_ERROR => Self::HardwareError,
            USER_ERROR => Self::UserError,
            OFF_LINE => Self::Offline,
            ON_LINE => Self::Online,
            _ => Self::HardwareError,
        }
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
            DeviceStatus::Changed => "changed",
            DeviceStatus::HardwareError => "hardware error",
            DeviceStatus::UserError => "user error",
            DeviceStatus::Offline => "offline",
            DeviceStatus::Online => "online",
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

impl_xfs_enum!(DeviceStatus, "deviceStatus");
