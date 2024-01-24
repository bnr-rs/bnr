use crate::{check_res, currency::CurrencyCode, Result};

mod cash_unit;
mod order;

pub use cash_unit::*;
pub use order::*;

/// Sends the initial message to start a `CashIn` transaction, and begin accepting notes.
pub fn cash_in_start() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_CashInStart() }, "cash_in_start")
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
    let mut amount = limit.unwrap_or(0);
    let amt_ptr = if limit.is_some() {
        &mut amount as *mut _
    } else {
        std::ptr::null_mut()
    };

    let mut cur = currency.map(<[i8; 4]>::from).unwrap_or([0i8; 4]);
    let cur_ptr = if currency.is_some() {
        &mut cur as *mut _
    } else {
        std::ptr::null_mut()
    };

    check_res(unsafe { bnr_sys::bnr_CashIn(amt_ptr, cur_ptr) }, "cash_in")
}

/// Sends the message to rollback a `CashIn` transaction, returning any inserted notes to the
/// customer.
///
/// The caller should first call the [cancel](crate::cancel) function to cancel the `CashIn`
/// transaction.
pub fn cash_in_rollback() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_CashInRollback() }, "cash_in_rollback")
}

/// Sends the message to end a `CashIn` transaction.
///
/// The caller will need to call [cash_in_start] and [cash_in] to begin accepting notes again.
pub fn cash_in_end() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_CashInEnd() }, "cash_in_end")
}

/// This command allows the application to force cash that has been presented to be ejected from the bezel.
pub fn eject() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_Eject() }, "eject")
}

/// Empties a recycler or loader cash unit in the cashbox.
///
/// **Note** When calling this function for a loader, the `to_float` parameter is not taken into account and the loader is completely emptied.
///
/// Params:
///
/// - `pcu_name`: Name of the physical cash unit to empty.
/// - `to_float` If `true`, the command empties up to the low threshold of the Physical Cash Unit, otherwise to zero.
pub fn empty(pcu_name: &PcuName, to_float: bool) -> Result<()> {
    check_res(
        unsafe { bnr_sys::bnr_Empty(pcu_name.clone().as_mut_ptr(), to_float.into()) },
        "empty",
    )
}

/// Gets the complete state of all physical and logical cash units in the BNR.
///
/// Returns the [CashUnit] struct with details about the [PhysicalCashUnit]s and
/// [LogicalCashUnit]s on the BNR device.
pub fn query_cash_unit() -> Result<CashUnit> {
    let mut cu = bnr_sys::XfsCashUnit::from(CashUnit::new());

    check_res(
        unsafe { bnr_sys::bnr_QueryCashUnit(&mut cu as *mut _) },
        "query_cash_unit",
    )?;

    Ok(cu.into())
}
