use std::fmt;

const HARDWARE_OK: u32 = 0;
const HARDWARE_MISSING: u32 = 1;
const HARDWARE_NOTIFICATION: u32 = 2;
const HARDWARE_WARNING: u32 = 3;
const HARDWARE_ERROR: u32 = 4;

/// Represents the overall status of the CDR hardware.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HardwareStatus {
    #[default]
    Ok = HARDWARE_OK,
    Missing = HARDWARE_MISSING,
    Notification = HARDWARE_NOTIFICATION,
    Warning = HARDWARE_WARNING,
    Error = HARDWARE_ERROR,
}

impl HardwareStatus {
    /// Creates a new [HardwareStatus].
    pub const fn new() -> Self {
        Self::Ok
    }

    /// Creates a new [HardwareStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            HARDWARE_OK => Self::Ok,
            HARDWARE_MISSING => Self::Missing,
            HARDWARE_NOTIFICATION => Self::Notification,
            HARDWARE_WARNING => Self::Warning,
            HARDWARE_ERROR => Self::Error,
            _ => Self::Missing,
        }
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

impl fmt::Display for HardwareStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(HardwareStatus, "hardwareStatus");
