use std::fmt;

use crate::impl_xfs_enum;

const REPORTING_NORMAL: u32 = 0;
const REPORTING_FULL: u32 = 1;

/// Defines the kind of error report to be generated when a failure is detected whith no bill being transported.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReportingMode {
    /// Normal reporting mode (default): on failure detection, a SimpleFailureReport will saved.
    #[default]
    Normal = REPORTING_NORMAL,
    /// Full reporting mode: on failure detection, a BillTransportErrorReport will saved.
    Full = REPORTING_FULL,
}

impl ReportingMode {
    /// Creates a new [ReportingMode].
    pub const fn new() -> Self {
        Self::Normal
    }

    /// Creates a new [ReportingMode] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            REPORTING_NORMAL => Self::Normal,
            REPORTING_FULL => Self::Full,
            _ => Self::Normal,
        }
    }
}

impl From<&ReportingMode> for &'static str {
    fn from(val: &ReportingMode) -> Self {
        match val {
            ReportingMode::Normal => "normal",
            ReportingMode::Full => "full",
        }
    }
}

impl From<ReportingMode> for &'static str {
    fn from(val: ReportingMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ReportingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(ReportingMode, "reportingMode");
