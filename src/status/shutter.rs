use super::HardwareStatus;

/// Represents shutter status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ShutterStatus {
    /// Indicates the shutter is closed
    Closed = bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_CLOSED,
    /// Indicates the shutter status is not supported
    NotSupported = bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_NOT_SUPPORTED,
    /// Indicates the shutter is open
    Open = bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_OPEN,
    /// Indicates the shutter status is unknown
    #[default]
    Unknown = bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_UNKNOWN,
}

impl ShutterStatus {
    /// Creates a new [ShutterStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for ShutterStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_CLOSED => Self::Closed,
            ds if ds == bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_NOT_SUPPORTED => {
                Self::NotSupported
            }
            ds if ds == bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_OPEN => Self::Open,
            ds if ds == bnr_sys::XfsCdrShutterStatus_XFS_S_CDR_SHT_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<ShutterStatus> for u32 {
    fn from(val: ShutterStatus) -> Self {
        val as u32
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
