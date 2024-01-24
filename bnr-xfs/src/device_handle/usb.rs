use futures_lite::future::block_on;
use nusb::transfer::{ControlIn, ControlType, Recipient, RequestBuffer};

use crate::xfs;
use crate::xfs::method_call::{XfsMethodCall, XfsMethodName};
use crate::xfs::method_response::{XfsMethodResponse, XfsMethodResponseStruct};
use crate::Result;

use super::*;

/// Represents a host-side USB device handle.
pub struct UsbDeviceHandle {
    device: nusb::Device,
    interface: nusb::Interface,
}

impl UsbDeviceHandle {
    /// Finds the BNR XFS USB device by PID:VID pair.
    pub fn find_usb() -> Result<Self> {
        let info = nusb::list_devices()
            .map_err(|err| {
                Error::Usb(format!("no devices found: {err}"))
            })?
        .find(|dev| {
            dev.vendor_id() == BNR_VID && dev.product_id() == BNR_PID
        })
        .ok_or(Error::Usb(format!("failed to find a USB device with the correct VID({BNR_VID:04x}):PID({BNR_PID:04x}) pair")))?;

        let device = info
            .open()
            .map_err(|err| Error::Usb(format!("unable to open device: {err}")))?;

        Self::setup_device(&device)?;

        let interface = device
            .claim_interface(0)
            .map_err(|err| Error::Usb(format!("unable to open main interface: {err}")))?;

        Ok(Self { device, interface })
    }

    /// Gets a reference to the USB [`Device`](nusb::Device).
    pub const fn device(&self) -> &nusb::Device {
        &self.device
    }

    /// Gets a reference to the USB [`Interface`](nusb::Interface).
    pub const fn interface(&self) -> &nusb::Interface {
        &self.interface
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_call(&self, call: &XfsMethodCall) -> Result<()> {
        block_on(
            self.interface
                .bulk_out(BNR_CALL_EP, xfs::to_string(call)?.into_bytes()),
        )
        .into_result()
        .map(|_| ())
        .map_err(|err| {
            let method = call.name_str();
            let err_msg = format!("error writing {method} message: {err}");
            log::warn!("{err_msg}");
            Error::Usb(err_msg)
        })
    }

    /// Reads an XFS method response (as a string) from the BNR response endpoint.
    pub fn read_response(&self, method: XfsMethodName) -> Result<XfsMethodResponse> {
        // Responses can be very large, so read from the endpoint in 4K chunks.
        let mut res_acc = Vec::with_capacity(4096);
        let mut res_buf = block_on(
            self.interface
                .bulk_in(BNR_RESPONSE_EP, RequestBuffer::new(4096)),
        )
        .into_result()
        .map_err(|err| {
            let err_msg = format!("Error reading {method} response: {err}");
            log::error!("{err_msg}");
            Error::Usb(err_msg)
        })?;

        let mut read = res_buf.len();
        res_acc.append(&mut res_buf);
        while read == 4096 {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf = match block_on(
                self.interface
                    .bulk_in(BNR_RESPONSE_EP, RequestBuffer::reuse(res_buf, 4096)),
            )
            .into_result()
            {
                Ok(r) => r,
                Err(_err) => Vec::new(),
            };
            read = res_buf.len();
            if read > 0 {
                res_acc.append(&mut res_buf);
            }
        }

        let res_str = std::str::from_utf8(res_acc.as_ref()).unwrap_or("");
        log::trace!("Raw {method} response: {res_str}");

        match xfs::from_str::<XfsMethodResponseStruct>(res_str) {
            Ok(r) => match r.into_inner() {
                XfsMethodResponse::Params(p) => {
                    log::debug!("BNR {method} response: {p}");
                    Ok(XfsMethodResponse::Params(p))
                }
                XfsMethodResponse::Fault(f) => {
                    let err_msg = format!("BNR {method} response fault: {f}");
                    log::warn!("{err_msg}");
                    Err(Error::Xfs(err_msg))
                }
            },
            Err(err) => {
                log::warn!("Error parsing {method} response: {err}");
                Err(err)
            }
        }
    }

    /// Reads an XFS callback call response (as a string) from the BNR response endpoint.
    pub fn read_callback_call(&self) -> Result<XfsMethodCall> {
        // Responses can be very large, so read from the endpoint in 4K chunks.
        let mut res_acc = Vec::with_capacity(4096);
        let mut res_buf = block_on(
            self.interface
                .bulk_in(BNR_CALLBACK_CALL_EP, RequestBuffer::new(4096)),
        )
        .into_result()
        .map_err(|err| {
            let err_msg = format!("Error reading callback call: {err}");
            log::error!("{err_msg}");
            Error::Usb(err_msg)
        })?;

        let mut read = res_buf.len();
        res_acc.append(&mut res_buf);
        while read == 4096 {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf = match block_on(
                self.interface
                    .bulk_in(BNR_CALLBACK_CALL_EP, RequestBuffer::reuse(res_buf, 4096)),
            )
            .into_result()
            {
                Ok(r) => r,
                Err(_err) => Vec::new(),
            };
            read = res_buf.len();
            if read > 0 {
                res_acc.append(&mut res_buf);
            }
        }

        let call_str = std::str::from_utf8(res_acc.as_ref()).unwrap_or("");
        log::trace!("Raw callback call: {call_str}");

        xfs::from_str::<XfsMethodCall>(call_str)
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_callback_response(
        &self,
        res: &XfsMethodResponse,
        name: XfsMethodName,
    ) -> Result<()> {
        let msg = xfs::to_iso_string(res)?.into_bytes();

        block_on(self.interface.bulk_out(BNR_CALLBACK_RESPONSE_EP, msg))
            .into_result()
            .map(|_| ())
            .map_err(|err| {
                let err_msg = format!("Error writing {name} callback response message: {err}");
                log::warn!("{err_msg}");
                Error::Usb(err_msg)
            })
    }

    fn setup_device(device: &nusb::Device) -> Result<()> {
        let exp_len: usize = 4;
        let ret: Vec<u8> = block_on(device.control_in(ControlIn {
            control_type: ControlType::Standard,
            recipient: Recipient::Device,
            request: 0x6,
            value: 0x3 << 8,
            index: 0x0,
            length: exp_len as u16,
        }))
        .into_result()?;

        let lang_id = if ret.len() >= exp_len {
            u16::from_le_bytes([ret[2], ret[3]])
        } else {
            0
        };

        if lang_id != 0 {
            log::debug!("Get language response: 0x{lang_id:04x}");

            let ret: Vec<u8> = block_on(device.control_in(ControlIn {
                control_type: ControlType::Standard,
                recipient: Recipient::Device,
                request: 0x6,
                value: 0x3 << 8 | 0x5,
                index: lang_id,
                length: 0xff,
            }))
            .into_result()?;

            for (i, lang) in ret
                .chunks_exact(2)
                .skip(1)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .enumerate()
            {
                log::debug!("Device language[{i}]: {lang:#06x}");
            }
        }

        Ok(())
    }
}
