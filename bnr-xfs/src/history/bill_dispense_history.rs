use crate::create_xfs_struct;
use crate::xfs::method_response::XfsMethodResponse;
use crate::{
    AmountNotAvailableCount, BillNotAvailableCount, BillRequestedCount,
    CashTypeDispenseHistoryItems, DenominateAmountCount, DirectFromLoaderCount,
    DispenseAmountCount, Error, Result, TooManyBillsCount,
};

create_xfs_struct!(
    BillDispenseHistory,
    "billDispenseHistory",
    [
        denominate_amount_count: DenominateAmountCount,
        amount_not_available_count: AmountNotAvailableCount,
        bill_requested_count: BillRequestedCount,
        bill_not_available_count: BillNotAvailableCount,
        too_many_bills_count: TooManyBillsCount,
        direct_from_loader_count: DirectFromLoaderCount,
        dispense_amount_count: DispenseAmountCount,
        cash_type_dispense_history_items: CashTypeDispenseHistoryItems
    ],
    "Represents the bill dispense history of the BNR device."
);

impl TryFrom<&XfsMethodResponse> for BillDispenseHistory {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected BillDispenseHistory XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for BillDispenseHistory {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
