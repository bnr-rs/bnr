use std::fmt;

use crate::impl_xfs_enum;

const THRESHOLD_SENSOR: u32 = 0;
const THRESHOLD_COUNT: u32 = 1;

/// Threshold mode used to determine the [ThresholdStatus] of a PCU.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ThresholdMode {
    /// [ThresholdStatus] changes are determined by the sensors of the BNR (physical filling status).
    #[default]
    Sensor = THRESHOLD_SENSOR,
    /// [ThresholdStatus] changes are determined by comparing the PCU counts to the Threshold levels.
    Count = THRESHOLD_COUNT,
}

impl ThresholdMode {
    /// Creates a new [ThresholdMode].
    pub const fn new() -> Self {
        Self::Sensor
    }

    /// Creates a new [ThresholdMode] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            THRESHOLD_SENSOR => Self::Sensor,
            THRESHOLD_COUNT => Self::Count,
            _ => Self::Sensor,
        }
    }
}

impl From<&ThresholdMode> for &'static str {
    fn from(val: &ThresholdMode) -> Self {
        match val {
            ThresholdMode::Sensor => "sensor",
            ThresholdMode::Count => "count",
        }
    }
}

impl From<ThresholdMode> for &'static str {
    fn from(val: ThresholdMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ThresholdMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(ThresholdMode, "thresholdMode");
