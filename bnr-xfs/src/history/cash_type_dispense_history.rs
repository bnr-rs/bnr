use crate::{create_xfs_array, create_xfs_struct};
use crate::{BillNotAvailableCount, BillRequestedCount, CashType};

const CASH_TYPE_HISTORY_DEFAULT: CashTypeDispenseHistory = CashTypeDispenseHistory::new();
const CASH_TYPE_HISTORY_LEN: usize = 61;

create_xfs_struct!(
    CashTypeDispenseHistory,
    "cashTypeDispenseHistory",
    [
        cash_type: CashType,
        bill_requested_count: BillRequestedCount,
        bill_not_available_count: BillNotAvailableCount
    ],
    "Represents the dispense history of each [CashType]"
);

create_xfs_array!(
    CashTypeDispenseHistoryItems,
    "cashTypeDispenseHistoryItems",
    CashTypeDispenseHistory,
    CASH_TYPE_HISTORY_LEN,
    CASH_TYPE_HISTORY_DEFAULT,
    "Represents a list of [CashTypeDispenseHistory] items."
);
