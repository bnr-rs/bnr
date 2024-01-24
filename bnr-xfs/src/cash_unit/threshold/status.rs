use std::fmt;

use crate::impl_xfs_enum;

const THRESHOLD_STAT_OK: u32 = 0;
const THRESHOLD_STAT_FULL: u32 = 1;
const THRESHOLD_STAT_HIGH: u32 = 2;
const THRESHOLD_STAT_LOW: u32 = 4;
const THRESHOLD_STAT_EMPTY: u32 = 8;
const THRESHOLD_STAT_UNKNOWN: u32 = 16;
const THRESHOLD_STAT_NOT_SUPPORTED: u32 = 32;

/// Filling status of a cash unit.
///
/// How the threshold status of a cash unit is determined, depends of the threshold mode :
///
/// @par SensorMode (default value):
/// ThresholdStatus changes are determined by the sensors of the BNR (physical filling status).
///
/// - `CountMode`: The management of these values depends of the physical cash unit type
///  - Cashbox, Recycler and Bundler: Based on the Threshold levels, but if the physical limit of filling is reached before the threshold Full, then the ThresholdStatus is forced to Full anyway.
///  - Loader: Based on the Threshold levels, but if there is a lack of bills before the threshold Empty, the ThresholdStatus is forced to Empty anyway.
///
/// **see** [Threshold], [ThresholdMode], [PhysicalCashUnit].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ThresholdStatus {
    /// The cash unit is neither empty nor full.
    #[default]
    Ok = THRESHOLD_STAT_OK,
    /// The cash unit is full. In CountMode, the cash unit count is higher or equal to its full threshold level.
    Full = THRESHOLD_STAT_FULL,
    /// The cash unit is alomst full. In CountMode, the cash unit count is higher than its high threshold level.
    High = THRESHOLD_STAT_HIGH,
    /// The cash unit is almost empty. In CountMode, the cash unit count is lower than its low threshold level.
    Low = THRESHOLD_STAT_LOW,
    /// The cash unit is empty. In CountMode, the cash unit count is lower or equal to its empty threshold level.
    Empty = THRESHOLD_STAT_EMPTY,
    /// Threshold state cannot be determined.
    Unknown = THRESHOLD_STAT_UNKNOWN,
    /// Threshold state is not supported.
    NotSupported = THRESHOLD_STAT_NOT_SUPPORTED,
}

impl ThresholdStatus {
    /// Creates a new [ThresholdStatus].
    pub const fn new() -> Self {
        Self::Ok
    }

    /// Creates a new [ThresholdStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            THRESHOLD_STAT_OK => Self::Ok,
            THRESHOLD_STAT_FULL => Self::Full,
            THRESHOLD_STAT_HIGH => Self::High,
            THRESHOLD_STAT_LOW => Self::Low,
            THRESHOLD_STAT_EMPTY => Self::Empty,
            THRESHOLD_STAT_UNKNOWN => Self::Unknown,
            THRESHOLD_STAT_NOT_SUPPORTED => Self::NotSupported,
            _ => Self::Unknown,
        }
    }
}

impl From<&ThresholdStatus> for &'static str {
    fn from(val: &ThresholdStatus) -> Self {
        match val {
            ThresholdStatus::Ok => "OK",
            ThresholdStatus::Full => "full",
            ThresholdStatus::High => "high",
            ThresholdStatus::Low => "low",
            ThresholdStatus::Empty => "empty",
            ThresholdStatus::Unknown => "unknown",
            ThresholdStatus::NotSupported => "not supported",
        }
    }
}

impl From<ThresholdStatus> for &'static str {
    fn from(val: ThresholdStatus) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ThresholdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(ThresholdStatus, "thresholdStatus");
