use super::*;
use crate::xfs::method_call::{XfsMethodCall, XfsMethodName};
use crate::xfs::method_response::{XfsMethodResponse, XfsMethodResponseStruct};
use crate::xfs::params::XfsParam;
use crate::xfs::value::XfsValue;

/// Class identifier of the `MainModule`.
pub const MAIN_MODULE_CLASS: i64 = 0xE0000;

impl DeviceHandle {
    pub(crate) fn open_inner(
        usb: UsbDeviceHandle,
        op_completed_callback: Option<OperationCompletedFn>,
        intermediate_occurred_callback: Option<IntermediateOccurredFn>,
        status_occurred_callback: Option<StatusOccurredFn>,
    ) -> Result<Self> {
        let call = XfsMethodCall::create(
            XfsMethodName::GetIdentification,
            [XfsParam::create_value(XfsValue::Int(MAIN_MODULE_CLASS))],
        );

        let timeout = time::Duration::from_millis(50);

        let languages = usb.read_languages(timeout)?;

        let mut lang_str = String::new();
        for (i, lang) in languages.iter().enumerate() {
            if i != 0 {
                lang_str += ", ";
            }
            let lid = lang.lang_id();
            lang_str += format!("{lid}").as_str();
        }

        if !languages.is_empty() {
            log::debug!("Get language response: [{lang_str}]");

            let desc = usb.read_string_descriptor(languages[0], 0, timeout)?;
            log::debug!("Interface string descriptor: {desc}");
        }

        let _read = match usb.read_interrupt(BNR_CALLBACK_CALL_EP, &mut [0u8; 4096], timeout) {
            Ok(r) => r,
            Err(err) => {
                let err_msg = format!("Error reading callback endpoint: {err}");
                log::warn!("{err_msg}");
                0
            }
        };

        // write the `getIdentification` call to the call endpoint
        Self::write_call(&usb, &call, timeout)?;

        // read the response
        Self::read_response(&usb, call.name()?, timeout).ok();

        Ok(Self {
            usb,
            op_completed_callback,
            intermediate_occurred_callback,
            status_occurred_callback,
        })
    }

    pub(crate) fn reset_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Reset);
        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn cancel_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Cancel);
        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn close_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::StopSession);
        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn reboot_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Reboot);
        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn get_date_time_inner(&self) -> Result<String> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetDateTime);
        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let res = Self::read_response(usb, call.name()?, timeout)?;
        let params = res.into_params()?;

        match params.params().iter().find(|p| p.is_param()) {
            Some(p) => Ok(p.as_param()?.as_value()?.as_date_time()?.into()),
            None => Err(Error::Xfs("expected DateTime param, none found".into())),
        }
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_call(
        usb: &UsbDeviceHandle,
        call: &XfsMethodCall,
        timeout: time::Duration,
    ) -> Result<()> {
        match usb.write_bulk(BNR_CALL_EP, xml::to_string(call)?.as_bytes(), timeout) {
            Ok(_) => Ok(()),
            Err(err) => {
                let method = call.name()?;
                let err_msg = format!("Error writing {method} message: {err}");
                log::warn!("{err_msg}");
                Err(err.into())
            }
        }
    }

    /// Reads an XFS method response (as a string) from the BNR response endpoint.
    pub fn read_response(
        usb: &UsbDeviceHandle,
        method: XfsMethodName,
        timeout: time::Duration,
    ) -> Result<XfsMethodResponse> {
        // Responses can be very large, so read from the endpoint in 4K chunks.
        let mut res_buf = [0u8; 4096];
        let mut res_str = String::with_capacity(4096);

        let mut read = match usb.read_bulk(BNR_RESPONSE_EP, &mut res_buf[..], timeout) {
            Ok(r) => r,
            Err(err) => {
                let err_msg = format!("Error reading {method} response: {err}");
                log::error!("{err_msg}");

                return Err(Error::Usb(err_msg));
            }
        };

        res_str += std::str::from_utf8(&res_buf[..read]).unwrap_or("");
        while read == res_buf.len() {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf.copy_from_slice([0u8; 4096].as_ref());
            read = match usb.read_bulk(BNR_RESPONSE_EP, &mut res_buf[..], timeout) {
                Ok(r) => r,
                Err(err) => {
                    let err_msg = format!("Error reading {method} response: {err}");
                    log::warn!("{err_msg}");

                    0
                }
            };
            res_str += std::str::from_utf8(&res_buf[..read]).unwrap_or("");
        }

        log::debug!("Raw {method} response: {res_str}");

        match xml::from_str::<XfsMethodResponseStruct>(res_str.as_str()) {
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
                Err(err.into())
            }
        }
    }
}
