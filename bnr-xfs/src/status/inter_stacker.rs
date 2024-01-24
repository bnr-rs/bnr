use std::fmt;

use super::HardwareStatus;

pub const CDR_IS_EMPTY: u32 = 6185;
pub const CDR_IS_NOT_EMPTY: u32 = 6186;
pub const CDR_IS_UNKNOWN: u32 = 6187;

/// Represents intermediate stacker status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IntermediateStackerStatus {
    /// Indicates the stacker is empty
    Empty = CDR_IS_EMPTY,
    /// Indicates the stacker is not empty
    NotEmpty = CDR_IS_NOT_EMPTY,
    /// Indicates the stacker status is unknown
    #[default]
    Unknown = CDR_IS_UNKNOWN,
}

impl IntermediateStackerStatus {
    /// Creates a new [IntermediateStackerStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [IntermediateStackerStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ds if ds == CDR_IS_EMPTY => Self::Empty,
            ds if ds == CDR_IS_NOT_EMPTY => Self::NotEmpty,
            ds if ds == CDR_IS_UNKNOWN => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<IntermediateStackerStatus> for HardwareStatus {
    fn from(val: IntermediateStackerStatus) -> Self {
        match val {
            IntermediateStackerStatus::Empty | IntermediateStackerStatus::NotEmpty => {
                Self::Notification
            }
            IntermediateStackerStatus::Unknown => Self::Warning,
        }
    }
}

impl From<IntermediateStackerStatus> for &'static str {
    fn from(val: IntermediateStackerStatus) -> Self {
        match val {
            IntermediateStackerStatus::Empty => "empty",
            IntermediateStackerStatus::NotEmpty => "not empty",
            IntermediateStackerStatus::Unknown => "unknown",
        }
    }
}

impl From<&IntermediateStackerStatus> for &'static str {
    fn from(val: &IntermediateStackerStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for IntermediateStackerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_xfs_enum!(IntermediateStackerStatus, "intermediateStackerStatus");
