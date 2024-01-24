use super::HardwareStatus;

/// Represents CDR dispenser status values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DispenserStatus {
    /// Cash unit changed
    Changed = bnr_sys::XFS_S_CDR_CASHUNIT_CHANGED,
    /// Cash unit configuration changed
    ConfigurationChanged = bnr_sys::XFS_S_CDR_CASHUNIT_CONFIGURATION_CHANGED,
    /// Cash unit threshold reached
    Threshold = bnr_sys::XFS_S_CDR_CASHUNIT_THRESHOLD,
    /// Cash available
    CashAvailable = bnr_sys::XFS_S_CDR_CASH_AVAILABLE,
    /// Cash taken
    CashTaken = bnr_sys::XFS_S_CDR_CASH_TAKEN,
    /// Indicates a cash unit state event
    State = bnr_sys::XFS_S_CDR_DIS_CU_STATE,
    /// Indicates a cash unit stop event
    Stop = bnr_sys::XFS_S_CDR_DIS_CU_STOP,
    /// Indicates an unknown cash unit event
    #[default]
    Unknown = bnr_sys::XFS_S_CDR_DIS_CU_UNKNOWN,
    /// Indicates the cash unit is OK
    Ok = bnr_sys::XFS_S_CDR_DIS_OK,
}

impl DispenserStatus {
    /// Creates a new [DispenserStatus].
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for DispenserStatus {
    fn from(val: u32) -> Self {
        match val {
            ds if ds == bnr_sys::XFS_S_CDR_CASHUNIT_CHANGED => Self::Changed,
            ds if ds == bnr_sys::XFS_S_CDR_CASHUNIT_CONFIGURATION_CHANGED => {
                Self::ConfigurationChanged
            }
            ds if ds == bnr_sys::XFS_S_CDR_CASHUNIT_THRESHOLD => Self::Threshold,
            ds if ds == bnr_sys::XFS_S_CDR_CASH_AVAILABLE => Self::CashAvailable,
            ds if ds == bnr_sys::XFS_S_CDR_CASH_TAKEN => Self::CashTaken,
            ds if ds == bnr_sys::XFS_S_CDR_DIS_CU_STATE => Self::State,
            ds if ds == bnr_sys::XFS_S_CDR_DIS_CU_STOP => Self::Stop,
            ds if ds == bnr_sys::XFS_S_CDR_DIS_CU_UNKNOWN => Self::Unknown,
            ds if ds == bnr_sys::XFS_S_CDR_DIS_OK => Self::Ok,
            _ => Self::Unknown,
        }
    }
}

impl From<DispenserStatus> for u32 {
    fn from(val: DispenserStatus) -> Self {
        val as u32
    }
}

impl From<DispenserStatus> for HardwareStatus {
    fn from(val: DispenserStatus) -> Self {
        match val {
            DispenserStatus::Ok => Self::Ok,
            DispenserStatus::Changed
            | DispenserStatus::ConfigurationChanged
            | DispenserStatus::CashAvailable
            | DispenserStatus::CashTaken
            | DispenserStatus::State => Self::Notification,
            DispenserStatus::Threshold | DispenserStatus::Stop | DispenserStatus::Unknown => {
                Self::Warning
            }
        }
    }
}
