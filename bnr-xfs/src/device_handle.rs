use std::time;

use serde_xml_rs as xml;

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

    /// Gets a reference to the [UsbDeviceHandle].
    pub const fn usb(&self) -> &UsbDeviceHandle {
        &self.usb
    }
}