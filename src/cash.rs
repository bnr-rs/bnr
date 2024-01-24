//! Functions for cash-related operations.

use bnr_xfs::{CurrencyCode, CashUnit, DispenseRequest, LogicalCashUnit, LogicalCashUnitList, PhysicalCashUnit, PhysicalCashUnitList, LCU_LIST_LEN, PCU_LIST_LEN};

use crate::{with_handle, Result};

/// Sends the initial message to start a `CashIn` transaction, and begin accepting notes.
pub fn cash_in_start() -> Result<()> {
    with_handle::<()>(|h| h.cash_in_start())
}

/// Sends the follow-up message to start a `CashIn` transaction, and begin accepting notes.
///
/// After successfully sending this message, the device is ready to accept notes.
///
/// params:
///
/// - `limit`: optional limit on the number of notes to accept.
///   - `None` will tell the device to accept one note.
/// - `currency`: optional restriction on currency to accept.
///   - `None` will tell the device to accept all currencies.
///
/// The BNR API takes two mutable pointers for this call, the first for `amount` and the second
/// for an ISO currency string (4-bytes, null-terminated ASCII).
///
/// From the BNR API docs:
///
/// ```no_build, no_run
/// @param[in] amount Amount to accept with this operation. If this parameter is NULL, the BNR
/// will accept only one banknote. If the amount is 0, banknotes will be accepted until the
/// escrow is full, or a bnr_Cancel() command is called. If the amount is different from 0,
/// banknotes will be accepted until the amount is reached, or the escrow is full, or a
/// bnr_Cancel() command is called.
///
/// @param[in] currencyCode Currency to accept during this operation. If this parameter is
/// NULL or the string is empty, any currency will be accepted by the BNR.
/// ```
pub fn cash_in(limit: Option<u32>, currency: Option<CurrencyCode>) -> Result<()> {
    with_handle::<()>(|h| h.cash_in(limit, currency))
}

/// Sends the message to rollback a `CashIn` transaction, returning any inserted notes to the
/// customer.
///
/// The caller should first call the [cancel](crate::cancel) function to cancel the `CashIn`
/// transaction.
pub fn cash_in_rollback() -> Result<()> {
    with_handle::<()>(|h| h.cash_in_rollback())
}

/// Sends the message to end a `CashIn` transaction.
///
/// The caller will need to call [cash_in_start] and [cash_in] to begin accepting notes again.
pub fn cash_in_end() -> Result<()> {
    with_handle::<()>(|h| h.cash_in_end())
}

/// This command allows the application to force cash that has been presented to be ejected from the bezel.
pub fn eject() -> Result<()> {
    with_handle::<()>(|h| h.eject())
}

/// Empties a recycler or loader cash unit in the cashbox.
///
/// **Note** When calling this function for a loader, the `to_float` parameter is not taken into account and the loader is completely emptied.
///
/// Params:
///
/// - `pcu_name`: Name of the physical cash unit to empty.
/// - `to_float` If `true`, the command empties up to the low threshold of the Physical Cash Unit, otherwise to zero.
pub fn empty(pcu_name: &str, to_float: bool) -> Result<()> {
    with_handle::<()>(|h| h.empty(pcu_name, to_float))
}

/// Gets the complete state of all physical and logical cash units in the BNR.
///
/// Returns the [CashUnit] struct with details about the [PhysicalCashUnit]s and
/// [LogicalCashUnit]s on the BNR device.
pub fn query_cash_unit() -> Result<CashUnit> {
    with_handle::<CashUnit>(|h| h.query_cash_unit())
}

/// Configures the BNR’s cash unit. This function is used to add or remove Logical and Physical Cash Unit in the BNR.
///
/// Those settings are persistent over power cycles.
///
/// Params:
///
/// - `transport_count`: number of bills in the transport system.
/// - `lcu_list`: [LogicalCashUnitList] for configuring [LogicalCashUnit]s.
/// - `pcu_list`: [PhysicalCashUnitList] for configuring [PhysicalCashUnit]s.
pub fn configure_cash_unit(
    transport_count: u32,
    lcu_list: &LogicalCashUnitList,
    pcu_list: &PhysicalCashUnitList,
) -> Result<(LogicalCashUnitList, PhysicalCashUnitList)> {
    with_handle::<(LogicalCashUnitList, PhysicalCashUnitList)>(|h| {
        h.configure_cash_unit(transport_count, lcu_list, pcu_list)?;
        Ok(h.query_cash_unit()?.into_lists())
    })
}

/// Updates the BNR’s cash unit. This function is used to change counts and thresholds of the BNR
/// [CashUnit]s.
///
/// Those settings are persistent over power cycles.
///
/// Params:
///
/// - `transport_count`: number of bills in the transport system.
/// - `lcu_list`: [LogicalCashUnitList] for configuring [LogicalCashUnit]s.
/// - `pcu_list`: [PhysicalCashUnitList] for configuring [PhysicalCashUnit]s.
pub fn update_cash_unit(
    transport_count: u32,
    lcu_list: &LogicalCashUnitList,
    pcu_list: &PhysicalCashUnitList,
) -> Result<()> {
    with_handle::<()>(|h| h.update_cash_unit(transport_count, lcu_list, pcu_list))
}

/// Resets the [LogicalCashUnit]s and [PhysicalCashUnit]s `count` to zero.
///
/// Useful for resetting device counters after physically removing notes from a device.
pub fn reset_cash_unit_counts() -> Result<()> {
    let cu = query_cash_unit()?;

    let lcu_keep: Vec<LogicalCashUnit> = cu
        .logical_cash_unit_list()
        .items()
        .iter()
        .filter(|l| l.count() != 0)
        .map(|l| l.with_count(0))
        .collect();

    let mut lcu_keep_list = [LogicalCashUnit::new(); LCU_LIST_LEN];

    let lcu_keep_len = lcu_keep.len();

    lcu_keep_list[..lcu_keep_len]
        .iter_mut()
        .zip(lcu_keep.iter())
        .for_each(|(dst, src)| {
            *dst = *src;
        });

    let lcu = LogicalCashUnitList::new()
        .with_size(lcu_keep_len as u32)
        .with_items(&lcu_keep_list[..lcu_keep_len]);

    let pcu_keep: Vec<PhysicalCashUnit> = cu
        .physical_cash_unit_list()
        .items()
        .iter()
        .filter(|p| p.count() != 0)
        .map(|p| p.with_count(0))
        .collect();

    let mut pcu_keep_list = [PhysicalCashUnit::new(); PCU_LIST_LEN];

    let pcu_keep_len = pcu_keep.len();

    pcu_keep_list[..pcu_keep_len]
        .iter_mut()
        .zip(pcu_keep.iter())
        .for_each(|(dst, src)| {
            *dst = *src;
        });

    let pcu = PhysicalCashUnitList::new()
        .with_size(pcu_keep_len as u32)
        .with_items(&pcu_keep_list[..pcu_keep_len]);

    update_cash_unit(0, &lcu, &pcu)
}

/// BNR_CASH_OPERATIONS Determines if the amount requested by value or by bill list, is available for dispense.
///
/// From the MEI/CPI documentation:
///
/// Three methods are possible:
///
/// - denominateRequest->mixNumber is #XFS_C_CDR_MXA_MIN_BILLS: The BNR chooses the banknotes to be distributed in order to obtain the total amount using the minimum number of banknotes. Two parameters must be correctly set:
///   - denominateRequest->denomination.amount has to be expressed in MDUs
///   - denominateRequest->currency.currencyCode is a string. See this page for a full list of the existing ISO currency codes: http://www.iso.org/iso/home/standards/currency_codes.htm
/// - denominateRequest->mixNumber is #BNRXFS_C_CDR_MXA_OPTIMUM_CHANGE: The BNR chooses the banknotes to be distributed in order to obtain the total amount in a way that slows down cashbox filling. As long as the low denomination Recyclers are not near to full, change is determined like with the MinBills algorithm. But when a Recycler becomes nearly full (5/6 of Full threshold), this algorithm will try to put more of this denomination in the change so that the Recycler doesn’t become full and this denomination doesn’t start to be cashed. Two parameters must be correctly set:
///    - denominateRequest->denomination.amount has to be expressed in MDUs
///    - denominateRequest->currency.currencyCode is a string. See this page for a full list of the existing ISO currency codes: http://www.iso.org/iso/home/standards/currency_codes.htm
/// - denominateRequest->mixNumber is #XFS_C_CDR_MIX_DENOM: The user chooses through a list of Logical Cash Units the banknotes to be distributed by the BNR in order to obtain the total amount. The following parameters must be correctly set:
///   - denominateRequest->denomination.size gives the size of the items array
///   - for each item of denominateRequest->denomination.items from 0 to (denominateRequest->denomination.size - 1):
///     - denominateRequest->denomination.items[item].unit contains the number of a LCU from where banknotes must be distributed.
///     - denominateRequest->denomination.items[item].count gives the number of banknotes to distribute from the LCU.
pub fn denominate(request: &DispenseRequest) -> Result<()> {
    with_handle::<()>(|h| h.denominate(request))
}

/// Dispenses the amount requested by value or by bill list.
///
/// From the MEI/CPI documentation:
///
/// The BNR will make a bundle of notes and wait for the bnr_Present() command to give it to the customer.
///
/// Three methods are possible:
///
/// - `DispenseRequest::mix_number` is #XFS_C_CDR_MXA_MIN_BILLS: The BNR chooses the banknotes to be distributed in order to obtain the total amount using the minimum number of banknotes. Two parameters must be correctly set:
///   - `DispenseRequest::denomination.amount` has to be expressed in MDUs
///   - `DispenseRequest::currency.currency_code`
///
/// - `DispenseRequest::mix_number` is #BNRXFS_C_CDR_MXA_OPTIMUM_CHANGE: The BNR chooses the banknotes to be distributed in order to obtain the total amount in a way that slows down cashbox filling. As long as the low denomination Recyclers are not near to full, change is determined like with the MinBills algorithm. But when a Recycler becomes nearly full (5/6 of Full threshold), this algorithm will try to put more of this denomination in the change so that the Recycler doesn’t become full and this denomination doesn’t start to be cashed. Two parameters must be correctly set:
///   - `DispenseRequest::denomination.amount` has to be expressed in MDUs
///   - `DispenseRequest::currency.currency_code`
///
/// - `DispenseRequest::mix_number` is #XFS_C_CDR_MIX_DENOM: The user chooses through a list of Logical Cash Units the banknotes to be distributed by the BNR in order to obtain the total amount. The following parameters must be correctly set:
///   - `DispenseRequest::denomination::size` gives the size of the items array
///       for each item of [DispenseRequest::denomination::items] from 0 to `DispenseRequest::denomination::size - 1`:
///      - `DispenseRequest::denomination::items[item]::unit` contains the number of a LCU from where banknotes must be distributed.
///      - `DispenseRequest::denomination::items[item]::count` gives the number of banknotes to distribute from the LCU.
///
/// - `DispenseRequest::currency.currency_code` is a string in the C library.
///   - See [CurrencyCode](crate::currency::CurrencyCode) for a full list of the existing ISO currency codes, also: <http://www.iso.org/iso/home/standards/currency_codes.htm>
///   - conversion from the enum to a string is handled internally, the user does not need to worry about this.
///
/// Params:
///
/// - `request`: Amount or bill list requested for dispense.
///
/// Returns `Ok` If function call is successful. Otherwise, return is strictly negative and its absolute value contains the error code.
pub fn dispense(request: &DispenseRequest) -> Result<()> {
    with_handle::<()>(|h| h.dispense(request))
}

/// Activates the presentation of the cash.
///
/// It can only be used following the [dispense] method.
///
/// A #XFS_S_CDR_CASH_AVAILABLE status event is issued to report that the bills are presented at the outlet,
/// then a #XFS_S_CDR_CASH_TAKEN status event is issued to report that the user has removed the bills, and the command completes.
///
/// After #XFS_S_CDR_CASH_AVAILABLE status event, if no #XFS_S_CDR_CASH_TAKEN status event is received within a reasonable time period,
/// the application should send a [cancel_waiting_cash_taken] to terminate the command, then send a [retract] to clear the bills from the outlet.
pub fn present() -> Result<()> {
    with_handle::<()>(|h| h.present())
}

/// Asks the BNR to stop waiting for cash removal at the Bezel if any.
///
/// If it can do so, an OperationCompleteEvent is sent with the result field containing #XFS_E_CANCELLED to indicate that the operation was cancelled.
/// Otherwise, the current operation’s messages will be sent as usual.
///
/// This method is meant to be called after the BNR has sent a #XFS_S_CDR_CASH_AVAILABLE status event, and before #XFS_S_CDR_CASH_TAKEN status event.
/// If this method is called outside these conditions, then no operation will take place and no error will be returned.
/// If this method is called after cash has been removed but before the #XFS_S_CDR_CASH_TAKEN status event has been returned to the caller,
/// then no operation will take place and no error will be returned.
pub fn cancel_waiting_cash_taken() -> Result<()> {
    with_handle::<()>(|h| h.cancel_waiting_cash_taken())
}

/// This command allows the application to force cash that has been presented to be retracted.
///
/// Retracted bills will be moved to the intermediate stacker area and accounted in the Bundler LCU. The application can then present bills to the user, using [cash_in_rollback] or [present]
/// (depending of the kind od the transaction) or clear the intermediate stacker area using the [reject] method.
///
/// This method may only be called after bills have been presented at the outlet following a [dispense] (if autoPresent mode is active), [cash_in_rollback] or [present] method call,
/// and before the bills have been taken by the user.
///
/// **Note** An asynchronous method must not be called before the preceding one is terminated (i.e. OperationComplete event has been received); typically before calling [retract],
/// the preceding command must be terminated by calling [cancel_waiting_cash_taken].
pub fn retract() -> Result<()> {
    with_handle::<()>(|h| h.retract())
}
