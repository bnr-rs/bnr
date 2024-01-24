use std::fmt;

use super::HardwareStatus;

pub const CT_EMPTY: u32 = 0;
pub const CT_NOT_EMPTY: u32 = 1;

/// Represents content status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContentStatus {
    /// Content is empty
    #[default]
    Empty = CT_EMPTY,
    /// Content is not empty
    NotEmpty = CT_NOT_EMPTY,
}

impl ContentStatus {
    /// Creates a new [ContentStatus].
    pub const fn new() -> Self {
        Self::Empty
    }

    /// Creates a new [ContentStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            ds if ds == CT_EMPTY => Self::Empty,
            ds if ds == CT_NOT_EMPTY => Self::NotEmpty,
            _ => Self::Empty,
        }
    }
}

impl From<ContentStatus> for HardwareStatus {
    fn from(val: ContentStatus) -> Self {
        match val {
            ContentStatus::Empty | ContentStatus::NotEmpty => Self::Notification,
        }
    }
}

impl From<&ContentStatus> for &'static str {
    fn from(val: &ContentStatus) -> Self {
        match val {
            ContentStatus::Empty => "empty",
            ContentStatus::NotEmpty => "not empty",
        }
    }
}

impl From<ContentStatus> for &'static str {
    fn from(val: ContentStatus) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ContentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_xfs_enum!(ContentStatus, "contentStatus");
