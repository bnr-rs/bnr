use crate::{create_xfs_array, create_xfs_struct};
use crate::{CashType, ForgeryCount};

use super::{ConfusionCount, FitnessRejectCount, SuspectCount, ValidCount, ValidUnfitCount};

pub const CASH_TYPE_HISTORY_LIST_LEN: usize = 61;
pub const CASH_TYPE_HISTORY_DEFAULT: CashTypeAcceptanceHistory = CashTypeAcceptanceHistory::new();

create_xfs_struct!(
    CashTypeAcceptanceHistory,
    "cashTypeAcceptanceHistory",
    [
        cash_type: CashType,
        confusion_count: ConfusionCount,
        forgery_count: ForgeryCount,
        fitness_reject_count: FitnessRejectCount,
        valid_count: ValidCount,
        valid_unfit_count: ValidUnfitCount,
        suspect_count: SuspectCount
    ],
    "Represents the history of cash acceptance events."
);

create_xfs_array!(
    CashTypeAcceptanceHistoryList,
    "cashTypeAcceptanceHistoryItems",
    CashTypeAcceptanceHistory,
    CASH_TYPE_HISTORY_LIST_LEN,
    CASH_TYPE_HISTORY_DEFAULT,
    "Represents a list of [CashTypeAcceptanceHistory] items."
);
