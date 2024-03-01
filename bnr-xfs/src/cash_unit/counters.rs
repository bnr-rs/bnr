use crate::create_xfs_struct;

mod counts;

pub use counts::*;

create_xfs_struct!(
    DepositCounters,
    "depositCounters",
    [
        deposit_count: DepositCount,
        retracted_count: RetractedCount,
        emptied_count: EmptiedCount,
        forgery_count: ForgeryCount,
        disappeared_count: DisappearedCount
    ],
    "Represents counters for deposits."
);

impl Copy for DepositCounters {}

create_xfs_struct!(
    DispenseCounters,
    "dispenseCounters",
    [dispense_count: DispenseCount, reject_count: RejectCount],
    "Represents counters for dispensed notes."
);

impl Copy for DispenseCounters {}

create_xfs_struct!(
    ExtendedCounters,
    "extendedCounters",
    [deposit: DepositCounters, dispense: DispenseCounters],
    "Represents extended counters for a cash unit."
);

impl From<DepositCounters> for ExtendedCounters {
    fn from(val: DepositCounters) -> Self {
        Self {
            deposit: val,
            dispense: DispenseCounters::new(),
        }
    }
}

impl From<&DepositCounters> for ExtendedCounters {
    fn from(val: &DepositCounters) -> Self {
        (*val).into()
    }
}

impl From<DispenseCounters> for ExtendedCounters {
    fn from(val: DispenseCounters) -> Self {
        Self {
            deposit: DepositCounters::new(),
            dispense: val,
        }
    }
}

impl From<&DispenseCounters> for ExtendedCounters {
    fn from(val: &DispenseCounters) -> Self {
        (*val).into()
    }
}

impl Copy for ExtendedCounters {}
