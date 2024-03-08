use crate::create_xfs_struct;
use crate::{
    BillRolledBackCount, CancelRejectCount, CashInTransactionCount, ConfigurationRejectCount,
    ConfusionCount, FitnessCount, ForgeryCount, InsertionRejectCount, InsertionStartCount,
    StainedCount, SuspectCount, TransportRejectCount, UnknownRejectCount, ValidCount,
    ValidUnfitCount,
};

create_xfs_struct!(
    InletAcceptanceHistory,
    "inletAcceptanceHistory",
    [
        insertion_start_count: InsertionStartCount,
        insertion_reject_count: InsertionRejectCount,
        cancel_reject_count: CancelRejectCount,
        unknown_reject_count: UnknownRejectCount,
        confusion_count: ConfusionCount,
        forgery_count: ForgeryCount,
        fitness_count: FitnessCount,
        valid_count: ValidCount,
        valid_unfit_count: ValidUnfitCount,
        suspect_count: SuspectCount,
        stained_count: StainedCount,
        configuration_reject_count: ConfigurationRejectCount,
        bill_rolled_back_count: BillRolledBackCount,
        cash_in_transaction_count: CashInTransactionCount,
        transport_reject_count: TransportRejectCount
    ],
    "Represents inlet acceptance history."
);
