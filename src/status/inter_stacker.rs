use std::fmt;

use super::HardwareStatus;

/// Represents intermediate stacker status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InterStackerStatus {
    /// Indicates the stacker is empty
    Empty = bnr_sys::XFS_S_CDR_IS_EMPTY,
    /// Indicates the stacker is not empty
    NotEmpty = bnr_sys::XFS_S_CDR_IS_NOT_EMPTY,
    /// Indicates the stacker status is unknown
    #[default]
    Unknown = bnr_sys::XFS_S_CDR_IS_UNKNOWN,
}

impl InterStackerStatus {
    /// Creates a new [InterStackerStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for InterStackerStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XFS_S_CDR_IS_EMPTY => Self::Empty,
            ds if ds == bnr_sys::XFS_S_CDR_IS_NOT_EMPTY => Self::NotEmpty,
            ds if ds == bnr_sys::XFS_S_CDR_IS_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<InterStackerStatus> for u32 {
    fn from(val: InterStackerStatus) -> Self {
        val as u32
    }
}

impl From<InterStackerStatus> for HardwareStatus {
    fn from(val: InterStackerStatus) -> Self {
        match val {
            InterStackerStatus::Empty | InterStackerStatus::NotEmpty => Self::Notification,
            InterStackerStatus::Unknown => Self::Warning,
        }
    }
}

impl From<InterStackerStatus> for &'static str {
    fn from(val: InterStackerStatus) -> Self {
        match val {
            InterStackerStatus::Empty => "empty",
            InterStackerStatus::NotEmpty => "not empty",
            InterStackerStatus::Unknown => "unknown",
        }
    }
}

impl From<&InterStackerStatus> for &'static str {
    fn from(val: &InterStackerStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for InterStackerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
