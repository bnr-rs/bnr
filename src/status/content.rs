use std::fmt;

use super::HardwareStatus;

/// Represents content status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContentStatus {
    /// Content is empty
    #[default]
    Empty = bnr_sys::XfsCdrContentStatus_XFS_S_CDR_CT_EMPTY,
    /// Content is not empty
    NotEmpty = bnr_sys::XfsCdrContentStatus_XFS_S_CDR_CT_NOT_EMPTY,
}

impl ContentStatus {
    /// Creates a new [ContentStatus].
    pub const fn new() -> Self {
        Self::Empty
    }
}

impl From<u32> for ContentStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XfsCdrContentStatus_XFS_S_CDR_CT_EMPTY => Self::Empty,
            ds if ds == bnr_sys::XfsCdrContentStatus_XFS_S_CDR_CT_NOT_EMPTY => Self::NotEmpty,
            _ => Self::Empty,
        }
    }
}

impl From<ContentStatus> for u32 {
    fn from(val: ContentStatus) -> Self {
        val as u32
    }
}

impl From<ContentStatus> for HardwareStatus {
    fn from(val: ContentStatus) -> Self {
        match val {
            ContentStatus::Empty | ContentStatus::NotEmpty => Self::Notification,
        }
    }
}

impl From<ContentStatus> for &'static str {
    fn from(val: ContentStatus) -> Self {
        match val {
            ContentStatus::Empty => "empty",
            ContentStatus::NotEmpty => "not empty",
        }
    }
}

impl From<&ContentStatus> for &'static str {
    fn from(val: &ContentStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ContentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
