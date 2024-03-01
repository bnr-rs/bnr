use crate::CashType;
use crate::{create_xfs_array, create_xfs_struct};

use super::StackedWhileRecyclerFullCount;

pub const CASH_TYPE_RECYCLE_LIST_LEN: usize = 10;
pub const CASH_TYPE_RECYCLE_DEFAULT: CashTypeRecycleHistoryItem = CashTypeRecycleHistoryItem::new();

create_xfs_struct!(
    CashTypeRecycleHistoryItem,
    "cashTypeRecycleHistoryItem",
    [
        cash_type: CashType,
        stacked_while_recycler_full_count: StackedWhileRecyclerFullCount
    ],
    "Represents a cash type recycle history item."
);

create_xfs_array!(
    CashTypeRecycleHistoryItems,
    "cashTypeRecycleHistoryItems",
    CashTypeRecycleHistoryItem,
    CASH_TYPE_RECYCLE_LIST_LEN,
    CASH_TYPE_RECYCLE_DEFAULT,
    "Represents a list of [CashTypeRecycleHistoryItem] items."
);
