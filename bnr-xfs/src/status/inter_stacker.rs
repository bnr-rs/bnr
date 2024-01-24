use std::fmt;

use super::HardwareStatus;

pub const CDR_IS_EMPTY: u32 = 6185;
pub const CDR_IS_NOT_EMPTY: u32 = 6186;
pub const CDR_IS_UNKNOWN: u32 = 6187;

/// Represents intermediate stacker status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InterStackerStatus {
    /// Indicates the stacker is empty
    Empty = CDR_IS_EMPTY,
    /// Indicates the stacker is not empty
    NotEmpty = CDR_IS_NOT_EMPTY,
    /// Indicates the stacker status is unknown
    #[default]
    Unknown = CDR_IS_UNKNOWN,
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
            ds if ds == CDR_IS_EMPTY => Self::Empty,
            ds if ds == CDR_IS_NOT_EMPTY => Self::NotEmpty,
            ds if ds == CDR_IS_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
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

impl_xfs_enum!(InterStackerStatus, "intermediateStackerStatus");
