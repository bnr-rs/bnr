use std::fmt;

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
        write!(f, "{}", <&str>::from(self))
    }
}
