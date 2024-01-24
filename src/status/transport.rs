use std::fmt;

use super::HardwareStatus;

/// Represents transport status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransportStatus {
    /// Indicates the transport is inoperable
    Inoperable = bnr_sys::XFS_S_CDR_TP_INOP,
    /// Indicates the transport is OK
    Ok = bnr_sys::XFS_S_CDR_TP_OK,
    /// Indicates the transport status is unknown
    #[default]
    Unknown = bnr_sys::XFS_S_CDR_TP_UNKNOWN,
    /// Indicates the transport status changed
    Changed = bnr_sys::XFS_S_CDR_TRANSPORT_CHANGED,
}

impl TransportStatus {
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for TransportStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XFS_S_CDR_TP_INOP => Self::Inoperable,
            ds if ds == bnr_sys::XFS_S_CDR_TP_OK => Self::Ok,
            ds if ds == bnr_sys::XFS_S_CDR_TP_UNKNOWN => Self::Unknown,
            ds if ds == bnr_sys::XFS_S_CDR_TRANSPORT_CHANGED => Self::Changed,
            _ => Self::Unknown,
        }
    }
}

impl From<TransportStatus> for u32 {
    fn from(val: TransportStatus) -> Self {
        val as u32
    }
}

impl From<TransportStatus> for HardwareStatus {
    fn from(val: TransportStatus) -> Self {
        match val {
            TransportStatus::Ok => Self::Ok,
            TransportStatus::Changed => Self::Notification,
            TransportStatus::Unknown => Self::Warning,
            TransportStatus::Inoperable => Self::Error,
        }
    }
}

impl From<TransportStatus> for &'static str {
    fn from(val: TransportStatus) -> Self {
        match val {
            TransportStatus::Ok => "OK",
            TransportStatus::Changed => "changed",
            TransportStatus::Unknown => "unknown",
            TransportStatus::Inoperable => "inoperable",
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
        write!(f, "{}", <&str>::from(self))
    }
}
