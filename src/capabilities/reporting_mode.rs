use std::fmt;

/// Defines the kind of error report to be generated when a failure is detected whith no bill being transported.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ReportingMode {
    /// Normal reporting mode (default): on failure detection, a SimpleFailureReport will saved.
    #[default]
    Normal = 0,
    /// Full reporting mode: on failure detection, a BillTransportErrorReport will saved.
    Full,
}

impl ReportingMode {
    /// Creates a new [ReportingMode].
    pub const fn new() -> Self {
        Self::Normal
    }
}

impl From<u32> for ReportingMode {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::Full,
            _ => Self::Normal,
        }
    }
}

impl From<ReportingMode> for u32 {
    fn from(val: ReportingMode) -> Self {
        val as u32
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
