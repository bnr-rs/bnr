use super::*;
use crate::xfs::{
    self,
    method_call::{XfsMethodCall, XfsMethodName},
    method_response::{XfsMethodResponse, XfsMethodResponseStruct},
    params::{XfsParam, XfsParams},
    value::XfsValue,
};

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
            [XfsParam::create(
                XfsValue::new().with_int(MAIN_MODULE_CLASS),
            )],
        );

        let timeout = time::Duration::from_millis(50);

        let mut ret = [0u8; 4];
        let read = usb.read_control(0x80, 0x6, 0x03 << 8, 0, &mut ret, timeout)?;

        let lang_id = if read == ret.len() {
            u16::from_le_bytes([ret[2], ret[3]])
        } else {
            0
        };

        if lang_id != 0 {
            log::debug!("Get language response: 0x{lang_id:04x}");

            let mut ret = [0u8; 0xff];
            let read = usb.read_control(0x80, 0x6, 0x3 << 8 | 0x5, lang_id, &mut ret, timeout)?;

            for (i, lang) in ret[..read]
                .chunks(2)
                .skip(1)
                .map(|c| u16::from_le_bytes(c.try_into().unwrap_or([0, 0])))
                .enumerate()
            {
                log::debug!("Interface lang[{i}]: {lang}");
            }
        }

        // FIXME: start a background thread to monitor device-sent events
        let _read = usb
            .read_interrupt(BNR_CALLBACK_CALL_EP, &mut [0u8; 4096], timeout)
            .ok();

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
            Err(Error::Xfs(err)) => {
                log::warn!("Error reading \"bnr.cancel\" response: {err}");
                Ok(())
            }
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
            Err(Error::Xfs(err)) => {
                log::warn!("Error reading \"bnr.stopsession\" response: {err}");
                Ok(())
            }
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
            Err(Error::Xfs(err)) => {
                log::warn!("Error reading \"bnr.reboot\" response: {err}");
                Ok(())
            }
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

        match params
            .params()
            .iter()
            .find(|p| p.inner().value().date_time().is_some())
        {
            Some(p) => Ok(p
                .inner()
                .value()
                .date_time()
                .ok_or(Error::Xfs("null param value".into()))?
                .into()),
            None => Err(Error::Xfs("expected DateTime param, none found".into())),
        }
    }

    pub(crate) fn set_date_time_inner(&self, date_time: datetime::OffsetDateTime) -> Result<()> {
        let iso_8601 =
            date_time.format(&datetime::format_description::well_known::Iso8601::DATE_TIME)?;

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::SetDateTime)
            .with_params(XfsParams::create([XfsParam::create(
                XfsValue::new().with_date_time(iso_8601),
            )]));

        let timeout = time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_call(
        usb: &UsbDeviceHandle,
        call: &XfsMethodCall,
        timeout: time::Duration,
    ) -> Result<()> {
        match usb.write_bulk(BNR_CALL_EP, xfs::to_string(call)?.as_bytes(), timeout) {
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
        let mut res_acc = Vec::with_capacity(4096);

        let mut read = match usb.read_bulk(BNR_RESPONSE_EP, &mut res_buf[..], timeout) {
            Ok(r) => r,
            Err(err) => {
                let err_msg = format!("Error reading {method} response: {err}");
                log::error!("{err_msg}");

                return Err(Error::Usb(err_msg));
            }
        };

        res_acc.extend_from_slice(&res_buf[..read]);
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
            res_acc.extend_from_slice(&res_buf[..read]);
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
                Err(err.into())
            }
        }
    }
}
