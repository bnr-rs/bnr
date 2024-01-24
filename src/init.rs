use std::ffi::c_void;

use crate::{check_res, Result};

/// Function signature for the `Operation Complete` callback used by the C API.
///
/// Indicates that an operation completed on the device.
///
/// The function crosses the FFI boundary, and is inherently unsafe. Care should be taken to use as
/// much safe Rust inside the callback as possible.
///
/// Casting the final `c_void` argument is the C way of achieving polymorphism. Special attention
/// should be paid to handling this pointer carefully, including checking for null, and determining
/// castable objects based on the other parameters.
///
/// The pointer is allocated by the C library, and the object it points to should be copied if sending
/// to another function for further handling.
///
/// Ownership of the pointer remains with the C library.
pub type OperationCompletedFn = unsafe extern "C" fn(i32, i32, i32, i32, *mut c_void);

/// Function signature for the `Intermediate Occured` callback used by the C API.
///
/// Indicates an intermediate state transition occured during an ongoing transaction.
///
/// The function crosses the FFI boundary, and is inherently unsafe. Care should be taken to use as
/// much safe Rust inside the callback as possible.
///
/// Casting the final `c_void` argument is the C way of achieving polymorphism. Special attention
/// should be paid to handling this pointer carefully, including checking for null, and determining
/// castable objects based on the other parameters.
///
/// The pointer is allocated by the C library, and the object it points to should be copied if sending
/// to another function for further handling.
///
/// Ownership of the pointer remains with the C library.
pub type IntermediateOccuredFn = unsafe extern "C" fn(i32, i32, i32, *mut c_void);

/// Function signature for the `Status Occured` callback used by the C API.
///
/// Indicates a status event occured on the device.
///
/// The function crosses the FFI boundary, and is inherently unsafe. Care should be taken to use as
/// much safe Rust inside the callback as possible.
///
/// Casting the final `c_void` argument is the C way of achieving polymorphism. Special attention
/// should be paid to handling this pointer carefully, including checking for null, and determining
/// castable objects based on the other parameters.
///
/// The pointer is allocated by the C library, and the object it points to should be copied if sending
/// to another function for further handling.
///
/// Ownership of the pointer remains with the C library.
pub type StatusOccuredFn = unsafe extern "C" fn(i32, i32, i32, *mut c_void);

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
    intermediate_occured_callback: Option<IntermediateOccuredFn>,
    status_occured_callback: Option<StatusOccuredFn>,
) -> Result<()> {
    check_res(
        unsafe {
            bnr_sys::bnr_Open(
                op_complete_callback,
                intermediate_occured_callback,
                status_occured_callback,
            )
        },
        "open",
    )
}

/// Sends the message to reset the device.
pub fn reset() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_Reset() }, "reset")
}

/// Sends the message to cancel any currently active transactions/commands.
pub fn cancel() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_Cancel() }, "cancel")
}

/// Stops secured communication session if started, ends the communication with the BNR and terminates the thread that has been started by a previous `open` call.
pub fn close() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_Close() }, "close")
}
