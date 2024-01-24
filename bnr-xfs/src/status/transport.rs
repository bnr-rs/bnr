use std::fmt;

use super::HardwareStatus;

pub const TRANSPORT_CHANGED: u32 = 6167;
pub const TRANSPORT_OK: u32 = 6203;
pub const TRANSPORT_INOP: u32 = 6204;
pub const TRANSPORT_UNKNOWN: u32 = 6206;

/// Represents transport status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransportStatus {
    /// Indicates the transport status changed
    Changed = TRANSPORT_CHANGED,
    /// Indicates the transport is OK
    Ok = TRANSPORT_OK,
    /// Indicates the transport is inoperable
    Inoperable = TRANSPORT_INOP,
    /// Indicates the transport status is unknown
    #[default]
    Unknown = TRANSPORT_UNKNOWN,
}

impl TransportStatus {
    /// Creates a new [TransportStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [TransportStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            TRANSPORT_CHANGED => Self::Changed,
            TRANSPORT_OK => Self::Ok,
            TRANSPORT_INOP => Self::Inoperable,
            TRANSPORT_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<TransportStatus> for HardwareStatus {
    fn from(val: TransportStatus) -> Self {
        match val {
            TransportStatus::Changed => Self::Notification,
            TransportStatus::Ok => Self::Ok,
            TransportStatus::Inoperable => Self::Error,
            TransportStatus::Unknown => Self::Warning,
        }
    }
}

impl From<TransportStatus> for &'static str {
    fn from(val: TransportStatus) -> Self {
        match val {
            TransportStatus::Changed => "changed",
            TransportStatus::Ok => "OK",
            TransportStatus::Inoperable => "inoperable",
            TransportStatus::Unknown => "unknown",
        }
    }
}

impl From<&TransportStatus> for &'static str {
    fn from(val: &TransportStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for TransportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(TransportStatus, "transportStatus");
