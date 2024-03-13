//! Functions related to device history.

use bnr_xfs::{
    BillAcceptanceHistory, BillDispenseHistory, SystemFailureHistory, SystemRestartHistory,
    SystemUseHistory,
};

use crate::{with_handle, Result};

/// Gets the BNR [`BillAcceptanceHistory`].
pub fn get_bill_acceptance_history() -> Result<BillAcceptanceHistory> {
    with_handle::<BillAcceptanceHistory>(|h| h.get_bill_acceptance_history())
}

/// Gets the BNR [`BillDispenseHistory`].
pub fn get_bill_dispense_history() -> Result<BillDispenseHistory> {
    with_handle::<BillDispenseHistory>(|h| h.get_bill_dispense_history())
}

/// Gets the BNR [`SystemFailureHistory`].
pub fn get_failure_history() -> Result<SystemFailureHistory> {
    with_handle::<SystemFailureHistory>(|h| h.get_failure_history())
}

/// Gets the BNR [`SystemRestartHistory`].
pub fn get_restart_history() -> Result<SystemRestartHistory> {
    with_handle::<SystemRestartHistory>(|h| h.get_restart_history())
}

/// Gets the BNR [`SystemUseHistory`].
pub fn get_use_history() -> Result<SystemUseHistory> {
    with_handle::<SystemUseHistory>(|h| h.get_use_history())
}
