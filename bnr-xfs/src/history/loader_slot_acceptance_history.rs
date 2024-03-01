use crate::{create_xfs_array, create_xfs_i4, create_xfs_struct};

use super::{
    BillExtractedCount, ExtractionRejectDetails, OtherDenominationCount, RecognitionRejectDetails,
    ValidUnfitCount,
};

pub const SLOT_HISTORY_LIST_LEN: usize = 4;
pub const LOADER_SLOT_DEFAULT: LoaderSlotAcceptanceHistory = LoaderSlotAcceptanceHistory::new();

create_xfs_i4!(SlotNumber, "slotNumber", "Represents the slot number.");

create_xfs_struct!(
    LoaderSlotAcceptanceHistory,
    "loaderSlotAcceptanceHistory",
    [
        slot_number: SlotNumber,
        bill_extracted_count: BillExtractedCount,
        other_denomination_count: OtherDenominationCount,
        valid_unfit_count: ValidUnfitCount,
        extraction_reject_details: ExtractionRejectDetails,
        recognition_reject_details: RecognitionRejectDetails
    ],
    "Represents the loader slot acceptance history."
);

create_xfs_array!(
    Slots,
    "slots",
    LoaderSlotAcceptanceHistory,
    SLOT_HISTORY_LIST_LEN,
    LOADER_SLOT_DEFAULT,
    "Represents a list of [LoaderSlotAcceptanceHistory] items."
);
