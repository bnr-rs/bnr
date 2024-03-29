use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

use time as datetime;

use crate::capabilities::Capabilities;
use crate::cash_unit::{CashUnit, LogicalCashUnitList, PhysicalCashUnitList};
use crate::currency::{CashOrder, CurrencyCode};
use crate::denominations::BillsetIdList;
use crate::denominations::DenominationList;
use crate::dispense::DispenseRequest;
use crate::history::{
    BillAcceptanceHistory, BillDispenseHistory, SystemFailureHistory, SystemRestartHistory,
    SystemUseHistory,
};
use crate::status::CdrStatus;
use crate::xfs;
use crate::{Error, Result};

mod inner;
pub mod usb;

use usb::UsbDeviceHandle;

/// BNR USB Vendor ID.
pub const BNR_VID: u16 = 0x0bed;
/// BNR USB Product ID.
pub const BNR_PID: u16 = 0x0a00;
/// Length of the URB (USB Request Block) header length on Linux.
pub const URB_LEN: u64 = 64;

/// BNR USB endpoint for device-to-host [XfsMethodResponse](crate::xfs::method_response::XfsMethodResponse)s.
///
/// Sets the MSB to indicate an `IN` endpoint.
pub const BNR_RESPONSE_EP: u8 = (1 << 7) | 1;
/// BNR USB endpoint for host-to-device [XfsMethodCall](crate::xfs::method_call::XfsMethodCall)s.
pub const BNR_CALL_EP: u8 = 2;
/// BNR USB endpoint for device-to-host asynchronous callback calls.
///
/// Sets the MSB to indicate an `IN` endpoint.
pub const BNR_CALLBACK_CALL_EP: u8 = (1 << 7) | 3;
/// BNR USB endpoint for host-to-device asynchronous callback responses.
pub const BNR_CALLBACK_RESPONSE_EP: u8 = 4;

/// Trait for arguments to state change callbacks used by the XFS API.
pub trait CallbackArg {
    fn value(&self) -> i32;
    fn is_null(&self) -> bool;
    fn is_cash_order(&self) -> bool;
    fn as_cash_order(&self) -> Result<&CashOrder>;
    fn as_cash_order_mut(&mut self) -> Result<&mut CashOrder>;
}

impl CallbackArg for () {
    fn value(&self) -> i32 {
        0
    }

    fn is_null(&self) -> bool {
        true
    }

    fn is_cash_order(&self) -> bool {
        false
    }

    fn as_cash_order(&self) -> Result<&CashOrder> {
        Err(Error::Xfs(
            "Expected CashOrder CallbackArg, have: null".into(),
        ))
    }

    fn as_cash_order_mut(&mut self) -> Result<&mut CashOrder> {
        Err(Error::Xfs(
            "Expected CashOrder CallbackArg, have: null".into(),
        ))
    }
}

/// Function signature for the `Operation Completed` callback used by the XFS API.
///
/// Handles device-sent messages indicating an asynchronous operation has completed.
///
/// # Parameters
///
/// - `call_id`: callback ID returned by the initial async call
/// - `operation_id`: async operation ID to uniquely identify the type of call
/// - `result`: the result status of the call
/// - `extended_result`: the extended result of the call
/// - `callback_arg`: callback call argument (may be the `unit` type if not supplied)
pub type OperationCompletedFn = fn(i32, i32, i32, i32, &mut dyn CallbackArg);

/// Function signature for the `Intermediate Occurred` callback used by the XFS API.
///
/// Handles an intermediate state transition occurred during an ongoing transaction.
///
/// # Parameters
///
/// - `call_id`: callback ID returned by the initial async call
/// - `operation_id`: async operation ID to uniquely identify the type of call
/// - `reason`: specifies the reason for the intermediate event
/// - `callback_arg`: callback call argument (may be the `unit` type if not supplied)
pub type IntermediateOccurredFn = fn(i32, i32, i32, &mut dyn CallbackArg);

/// Function signature for the `Status Occured` callback used by the XFS API.
///
/// Handles a status event that occurred on the device.
///
/// # Parameters
///
/// - `status`: the status that occurred on the device (e.g. bill inserted)
/// - `result`: the result of the status event
/// - `extended_result`: the extended result of the status event
/// - `callback_arg`: callback call argument (may be the `unit` type if not supplied)
pub type StatusOccurredFn = fn(i32, i32, i32, &mut dyn CallbackArg);

/// BNR XFS device handle for communication over USB.
pub struct DeviceHandle {
    usb: Arc<UsbDeviceHandle>,
    stop_listener: Arc<AtomicBool>,
    op_completed_callback: Option<OperationCompletedFn>,
    intermediate_occurred_callback: Option<IntermediateOccurredFn>,
    status_occurred_callback: Option<StatusOccurredFn>,
    response_rx: mpsc::Receiver<xfs::method_call::XfsMethodCall>,
}

impl DeviceHandle {
    /// Opens a new connection to the BNR XFS device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bnr_xfs::{CallbackArg, DeviceHandle};
    ///
    /// // Callback handler for when an async call completes
    /// //
    /// // See OperationCompletedFn for details.
    /// fn op_com(_call_id: i32, _op_id: i32, _res: i32, _ext_res: i32, _cb_arg: &mut dyn CallbackArg) {
    ///     // process the completion event...
    /// }
    ///
    /// // Callback handler for when an intermediate event occurs
    /// //
    /// // See IntermediateOccurredFn for details.
    /// fn int_oc(_call_id: i32, _op_id: i32, _reason: i32, _cb_arg: &mut dyn CallbackArg) {
    ///     // process the intermediate event...
    /// }
    ///
    /// // Callback handler for when a status event occurs
    /// //
    /// // See StatusOccurredFn for details.
    /// fn st_oc(_call_id: i32, _op_id: i32, _reason: i32, _cb_arg: &mut dyn CallbackArg) {
    ///     // process the status event...
    /// }
    ///
    /// let device_handle = DeviceHandle::open(Some(op_com), Some(int_oc), Some(st_oc)).unwrap();
    ///
    /// let _status = device_handle.get_status().unwrap();
    /// ```
    pub fn open(
        op_completed_callback: Option<OperationCompletedFn>,
        intermediate_occurred_callback: Option<IntermediateOccurredFn>,
        status_occurred_callback: Option<StatusOccurredFn>,
    ) -> Result<Self> {
        Self::open_inner(
            UsbDeviceHandle::find_usb()?,
            op_completed_callback,
            intermediate_occurred_callback,
            status_occurred_callback,
        )
    }

    /// Reconnects to the BNR XFS device
    pub fn reconnect(&mut self) -> Result<()> {
        self.stop_background_listener();
        self.usb = Arc::new(UsbDeviceHandle::find_usb()?);
        self.stop_listener.store(false, Ordering::SeqCst);
        let (response_tx, response_rx) = mpsc::channel();

        self.start_background_listener(response_tx, Arc::clone(&self.stop_listener))?;

        self.response_rx = response_rx;

        Ok(())
    }

    /// Resets the BNR device.
    pub fn reset(&self) -> Result<()> {
        self.reset_inner()
    }

    /// Sends the message to cancel any currently active transactions/commands.
    pub fn cancel(&self) -> Result<()> {
        self.cancel_inner()
    }

    /// Stops secured communication session if started, ends the communication with the BNR and terminates the thread that has been started by a previous `open` call.
    pub fn close(&self) -> Result<()> {
        self.close_inner()
    }

    /// Reboots the BNR. This call puts the BNR in the same state than a power cycle (power off/on).
    pub fn reboot(&self) -> Result<()> {
        self.reboot_inner()
    }

    /// Gets the ISO 8601 formatted date-time from the device.
    pub fn get_date_time(&self) -> Result<datetime::OffsetDateTime> {
        self.get_date_time_inner()
    }

    /// Sets the ISO 8601 formatted date-time on the device to the provided time.
    ///
    /// **NOTE** This setting is not persistent across reboots/power-cycles.
    ///
    /// The default device time will reset to `2001-01-01 00:00:00`.
    pub fn set_date_time(&self, date_time: datetime::OffsetDateTime) -> Result<()> {
        self.set_date_time_inner(date_time)
    }

    /// Sets the ISO 8601 formatted date-time on the device to the current time.
    ///
    /// **NOTE** This setting is not persistent across reboots/power-cycles.
    ///
    /// The default device time will reset to `2001-01-01 00:00:00`.
    pub fn set_current_date_time(&self) -> Result<()> {
        self.set_date_time_inner(datetime::OffsetDateTime::now_utc())
    }

    /// Gets the current status of the BNR device.
    pub fn get_status(&self) -> Result<CdrStatus> {
        self.get_status_inner()
    }

    /// "Parks" the device for maintenance, disabling all modules.
    pub fn park(&self) -> Result<()> {
        self.park_inner()
    }

    /// Gets the [Capabilities] of the BNR device.
    pub fn get_capabilities(&self) -> Result<Capabilities> {
        self.get_capabilities_inner()
    }

    /// Sets the [Capabilities] for the BNR device.
    pub fn set_capabilities(&self, caps: &Capabilities) -> Result<Capabilities> {
        self.set_capabilities_inner(caps)
    }

    /// Sends the initial message to start a `CashIn` transaction, and begin accepting notes.
    pub fn cash_in_start(&self) -> Result<()> {
        self.cash_in_start_inner()
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
    pub fn cash_in(&self, limit: Option<u32>, currency: Option<CurrencyCode>) -> Result<()> {
        self.cash_in_inner(limit, currency)
    }

    /// Sends the message to end a `CashIn` transaction.
    ///
    /// The caller will need to call [cash_in_start](Self::cash_in_start) and [cash_in](Self::cash_in) to begin accepting notes again.
    pub fn cash_in_end(&self) -> Result<()> {
        self.cash_in_end_inner()
    }

    /// Sends the message to rollback a `CashIn` transaction, returning any inserted notes to the
    /// customer.
    ///
    /// The caller should first call the [cancel](crate::cancel) function to cancel the `CashIn`
    /// transaction.
    pub fn cash_in_rollback(&self) -> Result<()> {
        self.cash_in_rollback_inner()
    }

    /// This command allows the application to force cash that has been presented to be ejected from the bezel.
    pub fn eject(&self) -> Result<()> {
        self.eject_inner()
    }

    /// Empties a recycler or loader cash unit in the cashbox.
    ///
    /// **Note** When calling this function for a loader, the `to_float` parameter is not taken into account and the loader is completely emptied.
    ///
    /// Params:
    ///
    /// - `pcu_name`: Name of the physical cash unit to empty.
    /// - `to_float` If `true`, the command empties up to the low threshold of the Physical Cash Unit, otherwise to zero.
    pub fn empty(&self, pcu_name: &str, to_float: bool) -> Result<()> {
        self.empty_inner(pcu_name, to_float)
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
    pub fn present(&self) -> Result<()> {
        self.present_inner()
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
    pub fn cancel_waiting_cash_taken(&self) -> Result<()> {
        self.cancel_waiting_cash_taken_inner()
    }

    /// This command allows the application to force cash that has been presented to be retracted.
    ///
    /// Retracted bills will be moved to the intermediate stacker area and accounted in the Bundler LCU. The application can then present bills to the user, using [cash_in_rollback](Self::cash_in_rollback) or [present](Self::present)
    /// (depending of the kind of the transaction) or clear the intermediate stacker area using the [reject](Self::reject) method.
    ///
    /// This method may only be called after bills have been presented at the outlet following a [dispense](Self::dispense) (if autoPresent mode is active), [cash_in_rollback](Self::cash_in_rollback) or [present](Self::present) method call,
    /// and before the bills have been taken by the user.
    ///
    /// **Note** An asynchronous method must not be called before the preceding one is terminated (i.e. OperationComplete event has been received); typically before calling [retract],
    /// the preceding command must be terminated by calling
    /// [cancel_waiting_cash_taken](Self::cancel_waiting_cash_taken).
    pub fn retract(&self) -> Result<()> {
        self.retract_inner()
    }

    /// Gets the complete state of all physical and logical cash units in the BNR.
    ///
    /// Returns the [CashUnit] struct with details about the [PhysicalCashUnit]s and
    /// [LogicalCashUnit]s on the BNR device.
    pub fn query_cash_unit(&self) -> Result<CashUnit> {
        self.query_cash_unit_inner()
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
        &self,
        transport_count: u32,
        lcu_list: &LogicalCashUnitList,
        pcu_list: &PhysicalCashUnitList,
    ) -> Result<()> {
        self.configure_cash_unit_inner(transport_count, lcu_list, pcu_list)
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
        &self,
        transport_count: u32,
        lcu_list: &LogicalCashUnitList,
        pcu_list: &PhysicalCashUnitList,
    ) -> Result<()> {
        self.update_cash_unit_inner(transport_count, lcu_list, pcu_list)
    }

    /// BNR_CASH_OPERATIONS Determines if the amount requested by value or by bill list, is available for dispense.
    ///
    /// From the MEI/CPI documentation:
    ///
    /// Three methods are possible:
    ///
    /// - denominateRequest->mixNumber is #XFS_C_CDR_MXA_MIN_BILLS: The BNR chooses the banknotes to be distributed in order to obtain the total amount using the minimum number of banknotes. Two parameters must be correctly set:
    ///   - denominateRequest->denomination.amount has to be expressed in MDUs
    ///   - denominateRequest->currency.currencyCode is a string. See this page for a full list of the existing ISO currency codes: <http://www.iso.org/iso/home/standards/currency_codes.htm>
    /// - denominateRequest->mixNumber is #BNRXFS_C_CDR_MXA_OPTIMUM_CHANGE: The BNR chooses the banknotes to be distributed in order to obtain the total amount in a way that slows down cashbox filling. As long as the low denomination Recyclers are not near to full, change is determined like with the MinBills algorithm. But when a Recycler becomes nearly full (5/6 of Full threshold), this algorithm will try to put more of this denomination in the change so that the Recycler doesn’t become full and this denomination doesn’t start to be cashed. Two parameters must be correctly set:
    ///    - denominateRequest->denomination.amount has to be expressed in MDUs
    ///    - denominateRequest->currency.currencyCode is a string. See this page for a full list of the existing ISO currency codes: <http://www.iso.org/iso/home/standards/currency_codes.htm>
    /// - denominateRequest->mixNumber is #XFS_C_CDR_MIX_DENOM: The user chooses through a list of Logical Cash Units the banknotes to be distributed by the BNR in order to obtain the total amount. The following parameters must be correctly set:
    ///   - denominateRequest->denomination.size gives the size of the items array
    ///   - for each item of denominateRequest->denomination.items from 0 to (denominateRequest->denomination.size - 1):
    ///     - denominateRequest->denomination.items[item].unit contains the number of a LCU from where banknotes must be distributed.
    ///     - denominateRequest->denomination.items[item].count gives the number of notes to distribute from the LCU.
    pub fn denominate(&self, request: &DispenseRequest) -> Result<()> {
        self.denominate_inner(request)
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
    ///   - See [CurrencyCode] for a full list of the existing ISO currency codes, also: <http://www.iso.org/iso/home/standards/currency_codes.htm>
    ///   - conversion from the enum to a string is handled internally, the user does not need to worry about this.
    ///
    /// Params:
    ///
    /// - `request`: Amount or bill list requested for dispense.
    ///
    /// Returns `Ok` If function call is successful. Otherwise, return is strictly negative and its absolute value contains the error code.
    pub fn dispense(&self, request: &DispenseRequest) -> Result<()> {
        self.dispense_inner(request)
    }

    /// Stops any active sessions on the BNR device.
    pub fn stop_session(&self) -> Result<()> {
        self.stop_session_inner()
    }

    /// Gets a list of denominations in the BNR.
    ///
    /// Returns:
    ///
    /// - Ok([DenominationList]): list of the denominations currently defined in the BNR.
    /// - Error conditions: see [update_denominations](Self::update_denominations) for a list of error code descriptions.
    pub fn query_denominations(&self) -> Result<DenominationList> {
        self.query_denominations_inner()
    }

    /// Updates the settings for a list of denominations.
    ///
    /// For each [DenominationInfo](crate::denominations::DenominationInfo) element of the [DenominationList],
    /// the application can update its validation settings.
    ///
    /// From the BNR API docs:
    ///
    /// ```no_build,no_run
    /// Those settings are persistent over power cycles; please refer to DenominationInfo for more details about settable properties, and their default values.
    ///
    /// @param[in] DenominationList This list of denominations will be a modified version of the one obtained from query_denominations() call.
    /// ```
    ///
    /// Returns:
    ///
    /// - Ok(()) on success
    /// - Error conditions:
    ///   - `#XFS_E_ILLEGAL` - A dispense command is already active on the BNR.
    ///   - `#XFS_E_NOT_SUPPORTED` - operation not supported by the BNR firmware version.
    ///   - `#XFS_E_PARAMETER_INVALID` - Invalid array size. The array size is bigger than expected.
    ///   - `#XFS_E_CDR_CASHIN_ACTIVE` - A cashIn command has been issued and is already active.
    ///   - `#XFS_E_FAILURE` - a command is already running on the BNR or an internal error occured.
    pub fn update_denominations(&self, request: &DenominationList) -> Result<()> {
        self.update_denominations_inner(request)
    }

    /// Queries the device for the configured [BillsetIdList].
    ///
    /// **NOTE** Firmware Compatibility: This function requires a BNR FW v1.12.0 or newer. With older FW versions, the return will be #XFS_E_NOT_SUPPORTED.
    pub fn query_billset_ids(&self) -> Result<BillsetIdList> {
        self.query_billset_ids_inner()
    }

    /// Gets the BNR [BillAcceptanceHistory].
    pub fn get_bill_acceptance_history(&self) -> Result<BillAcceptanceHistory> {
        self.get_bill_acceptance_history_inner()
    }

    /// Gets the BNR [BillDispenseHistory].
    pub fn get_bill_dispense_history(&self) -> Result<BillDispenseHistory> {
        self.get_bill_dispense_history_inner()
    }

    /// Gets the BNR [SystemFailureHistory].
    pub fn get_failure_history(&self) -> Result<SystemFailureHistory> {
        self.get_failure_history_inner()
    }

    /// Gets the BNR [SystemRestartHistory].
    pub fn get_restart_history(&self) -> Result<SystemRestartHistory> {
        self.get_restart_history_inner()
    }

    /// Gets the BNR [SystemUseHistory].
    pub fn get_use_history(&self) -> Result<SystemUseHistory> {
        self.get_use_history_inner()
    }

    /// Gets a reference to the [UsbDeviceHandle].
    pub(crate) fn usb(&self) -> &UsbDeviceHandle {
        self.usb.as_ref()
    }

    pub(crate) fn usb_clone(&self) -> Arc<UsbDeviceHandle> {
        Arc::clone(&self.usb)
    }

    /// Gets the callback function for operation completed events.
    pub fn op_completed_callback(&self) -> Option<OperationCompletedFn> {
        self.op_completed_callback
    }

    /// Gets the callback function for intermediate events.
    pub fn intermediate_occurred_callback(&self) -> Option<IntermediateOccurredFn> {
        self.intermediate_occurred_callback
    }

    /// Gets the callback function for status events.
    pub fn status_occurred_callback(&self) -> Option<StatusOccurredFn> {
        self.status_occurred_callback
    }
}
