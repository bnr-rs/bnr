use std::fmt;

use super::HardwareStatus;

pub const CASH_UNIT_CHANGED: u32 = 6153;
pub const CASH_UNIT_CONFIG_CHANGED: u32 = 6154;
pub const CASH_UNIT_THRESHOLD: u32 = 6155;
pub const CASH_UNIT_OK: u32 = 6181;
pub const CASH_UNIT_STATE: u32 = 6182;
pub const CASH_UNIT_STOP: u32 = 6183;
pub const CASH_UNIT_UNKNOWN: u32 = 6184;
pub const CASH_TAKEN: u32 = 6192;
pub const CASH_AVAILABLE: u32 = 6223;

/// Represents CDR dispenser status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DispenserStatus {
    /// Cash unit changed
    Changed = CASH_UNIT_CHANGED,
    /// Cash unit configuration changed
    ConfigChanged = CASH_UNIT_CONFIG_CHANGED,
    /// Cash unit threshold reached
    Threshold = CASH_UNIT_THRESHOLD,
    /// Indicates the cash unit is OK
    Ok = CASH_UNIT_OK,
    /// Indicates a cash unit state event
    State = CASH_UNIT_STATE,
    /// Indicates a cash unit stop event
    Stop = CASH_UNIT_STOP,
    /// Indicates an unknown cash unit event
    #[default]
    Unknown = CASH_UNIT_UNKNOWN,
    /// Cash taken
    CashTaken = CASH_TAKEN,
    /// Cash available
    CashAvailable = CASH_AVAILABLE,
}

impl DispenserStatus {
    /// Creates a new [DispenserStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [DispenserStatus] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            CASH_UNIT_CHANGED => Self::Changed,
            CASH_UNIT_CONFIG_CHANGED => Self::ConfigChanged,
            CASH_UNIT_THRESHOLD => Self::Threshold,
            CASH_UNIT_OK => Self::Ok,
            CASH_UNIT_STATE => Self::State,
            CASH_UNIT_STOP => Self::Stop,
            CASH_UNIT_UNKNOWN => Self::Unknown,
            CASH_TAKEN => Self::CashTaken,
            CASH_AVAILABLE => Self::CashAvailable,
            _ => Self::Unknown,
        }
    }
}

impl From<DispenserStatus> for HardwareStatus {
    fn from(val: DispenserStatus) -> Self {
        match val {
            DispenserStatus::Ok => Self::Ok,
            DispenserStatus::Changed
            | DispenserStatus::ConfigChanged
            | DispenserStatus::CashAvailable
            | DispenserStatus::CashTaken
            | DispenserStatus::State => Self::Notification,
            DispenserStatus::Threshold | DispenserStatus::Stop | DispenserStatus::Unknown => {
                Self::Warning
            }
        }
    }
}

impl From<DispenserStatus> for &'static str {
    fn from(val: DispenserStatus) -> Self {
        match val {
            DispenserStatus::Changed => "changed",
            DispenserStatus::ConfigChanged => "configuration changed",
            DispenserStatus::Threshold => "threshold",
            DispenserStatus::CashAvailable => "cash available",
            DispenserStatus::CashTaken => "cash taken",
            DispenserStatus::State => "state",
            DispenserStatus::Stop => "stop",
            DispenserStatus::Unknown => "unknown",
            DispenserStatus::Ok => "OK",
        }
    }
}

impl From<&DispenserStatus> for &'static str {
    fn from(val: &DispenserStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for DispenserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_xfs_enum!(DispenserStatus, "dispenserStatus");
