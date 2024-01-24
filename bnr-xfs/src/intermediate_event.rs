use std::fmt;

use crate::impl_xfs_enum;

const INTERMEDIATE_CDR_OFFSET: u32 = 6140;
const CDR_INPUT_REFUSED: u32 = 6209;
const CDR_SUBCASHIN: u32 = INTERMEDIATE_CDR_OFFSET;
const CDR_BCC_INSERTED: u32 = INTERMEDIATE_CDR_OFFSET + 1;
const CDR_NOT_SUPPORTED: u32 = u32::MAX;

/// Cash Module intermediate event.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IntermediateEvent {
    /// Input refused.
    InputRefused = CDR_INPUT_REFUSED,
    /// A bill has been recognized during a cash in transaction, but the amount requested has not been reached yet.
    #[default]
    SubCashIn = CDR_SUBCASHIN,
    /// A coupon with barcode has been inserted and recognized during a cash in transaction, the BNR then waits for a [`set_recognition_result()`](crate::DeviceHandle::set_recognition_result) call.
    BccInserted = CDR_BCC_INSERTED,
    /// Unsupported [IntermediateEvent] number.
    NotSupported = CDR_NOT_SUPPORTED,
}

impl IntermediateEvent {
    /// Creates a new [IntermediateEvent].
    pub const fn new() -> Self {
        Self::SubCashIn
    }

    /// Creates a new [IntermediateEvent] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            CDR_INPUT_REFUSED => Self::InputRefused,
            CDR_SUBCASHIN => Self::SubCashIn,
            CDR_BCC_INSERTED => Self::BccInserted,
            _ => Self::NotSupported,
        }
    }
}

impl From<&IntermediateEvent> for &'static str {
    fn from(val: &IntermediateEvent) -> Self {
        match val {
            IntermediateEvent::InputRefused => "input refused",
            IntermediateEvent::SubCashIn => "sub cash in",
            IntermediateEvent::BccInserted => "BCC inserted",
            IntermediateEvent::NotSupported => "not supported",
        }
    }
}

impl fmt::Display for IntermediateEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(IntermediateEvent, "intermediateEvent");
