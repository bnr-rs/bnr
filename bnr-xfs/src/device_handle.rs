use time as datetime;

use crate::capabilities::Capabilities;
use crate::currency::CurrencyCode;
use crate::status::CdrStatus;
use crate::{Error, Result};

mod inner;

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

/// Convenience alias for [USB device handle](rusb::DeviceHandle).
pub type UsbDeviceHandle = rusb::DeviceHandle<rusb::Context>;

/// Trait for arguments to state change callbacks used by the XFS API.
pub trait CallbackArg {
    fn value(&self) -> i32;
}

/// Function signature for the `Operation Completed` callback used by the XFS API.
///
/// Handles device-sent messages indicating an asynchronous operation has completed.
pub type OperationCompletedFn = fn(i32, i32, i32, i32, &mut dyn CallbackArg);

/// Function signature for the `Intermediate Occurred` callback used by the XFS API.
///
/// Handles an intermediate state transition occurred during an ongoing transaction.
pub type IntermediateOccurredFn = fn(i32, i32, i32, &mut dyn CallbackArg);

/// Function signature for the `Status Occured` callback used by the XFS API.
///
/// Handles a status event that occurred on the device.
pub type StatusOccurredFn = fn(i32, i32, i32, &mut dyn CallbackArg);

/// BNR XFS device handle for communication over USB.
pub struct DeviceHandle {
    usb: UsbDeviceHandle,
    op_completed_callback: Option<OperationCompletedFn>,
    intermediate_occurred_callback: Option<IntermediateOccurredFn>,
    status_occurred_callback: Option<StatusOccurredFn>,
}

impl DeviceHandle {
    /// Opens a new connection to the BNR XFS device.
    pub fn open(
        op_completed_callback: Option<OperationCompletedFn>,
        intermediate_occurred_callback: Option<IntermediateOccurredFn>,
        status_occurred_callback: Option<StatusOccurredFn>,
    ) -> Result<Self> {
        let ctx = rusb::Context::new()?;
        let dev_list = rusb::DeviceList::new_with_context(ctx)?;

        let dev = dev_list.iter().find(|d| {
            if let Ok(desc) = d.device_descriptor() {
                desc.vendor_id() == BNR_VID && desc.product_id() == BNR_PID
            } else {
                false
            }
        });

        match dev {
            Some(usb) => Self::open_inner(usb.open()?, op_completed_callback, intermediate_occurred_callback, status_occurred_callback),
            None => Err(Error::Usb(format!("failed to find a USB device with the correct VID({BNR_VID:04x}):PID({BNR_PID:04x}) pair"))),
        }
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

    /// Gets the [Capabilities](crate::Capabilities) of the BNR device.
    pub fn get_capabilities(&self) -> Result<Capabilities> {
        self.get_capabilities_inner()
    }

    /// Sets the [Capabilities](crate::status::CdrPositionCapabilitiesList) for the BNR device.
    pub fn set_capabilities(&self, caps: &Capabilities) -> Result<()> {
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

    /// Gets a reference to the [UsbDeviceHandle].
    pub const fn usb(&self) -> &UsbDeviceHandle {
        &self.usb
    }
}
