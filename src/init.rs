use crate::{deinit_handle, init_handle, with_handle, DeviceHandle, Result};

use bnr_xfs::{IntermediateOccurredFn, OperationCompletedFn, StatusOccurredFn};

/// Sends the message to open the device.
///
/// Issues a request through the USB to determine if a BNR is connected, and if a BNR is connected and powered,
/// starts the thread to monitor the BNR and to communicate with it.
///
/// Stops secured communication session to make sure that secured session is not active after starting communication.
///
/// This function takes three callback functions as parameters to handle the different events the BNR can send.
pub fn open(
    op_complete_callback: Option<OperationCompletedFn>,
    intermediate_occurred_callback: Option<IntermediateOccurredFn>,
    status_occurred_callback: Option<StatusOccurredFn>,
) -> Result<()> {
    let handle = DeviceHandle::open(
        op_complete_callback,
        intermediate_occurred_callback,
        status_occurred_callback,
    )?;

    init_handle(handle)
}

/// Sends the message to reset the device.
pub fn reset() -> Result<()> {
    with_handle::<()>(|h| h.reset())
}

/// Sends the message to cancel any currently active transactions/commands.
pub fn cancel() -> Result<()> {
    with_handle::<()>(|h| h.cancel())
}

/// Stops secured communication session if started, ends the communication with the BNR and terminates the thread that has been started by a previous `open` call.
pub fn close() -> Result<()> {
    with_handle::<()>(|h| h.close())
}

/// Reboots the BNR. This call puts the BNR in the same state than a power cycle (power off/on).
pub fn reboot() -> Result<()> {
    with_handle::<()>(|h| h.reboot())
}

/// De-initializes the global [DeviceHandle] instance.
///
/// **NOTE** users are required to call either [DeviceHandle::open] or [open()] before further BNR
/// device communication.
pub fn destroy() -> Result<()> {
    deinit_handle()
}
