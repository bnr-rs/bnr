use datetime::format_description::well_known::{
    iso8601::{Config, TimePrecision},
    Iso8601,
};
use time as datetime;

use super::*;
use crate::capabilities::Capabilities;
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

        let timeout = std::time::Duration::from_millis(50);

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
        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn cancel_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Cancel);
        let timeout = std::time::Duration::from_millis(50);
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
        let timeout = std::time::Duration::from_millis(50);
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
        let timeout = std::time::Duration::from_millis(50);
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

    pub(crate) fn get_date_time_inner(&self) -> Result<datetime::OffsetDateTime> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetDateTime);
        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let res = Self::read_response(usb, call.name()?, timeout)?;
        let params = res.into_params()?;

        let date_res = params
            .params()
            .iter()
            .find(|p| p.inner().value().date_time().is_some())
            .ok_or(Error::Xfs("expected DateTime param, none found".into()))?;

        let date_str = date_res
            .inner()
            .value()
            .date_time()
            .ok_or(Error::Xfs("null param value".into()))?;

        if date_str.len() < 17 {
            Err(Error::DateTime(format!(
                "invalid ISO-8601 DateTime, too short: {date_str}"
            )))
        } else {
            let mut date_string = date_str.to_string();

            date_string.insert(6, '-');
            date_string.insert(4, '-');

            date_string += "+00:00";

            log::debug!("Formatted DateTime: {date_string}");

            Ok(datetime::OffsetDateTime::parse(
                date_string.as_str(),
                &Iso8601::DATE_TIME,
            )?)
        }
    }

    pub(crate) fn set_date_time_inner(&self, date_time: datetime::OffsetDateTime) -> Result<()> {
        let date_fmt = Iso8601::<
            {
                Config::DEFAULT
                    .set_time_precision(TimePrecision::Second {
                        decimal_digits: None,
                    })
                    .encode()
            },
        >;

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::SetDateTime)
            .with_params(XfsParams::create([XfsParam::create(
                XfsValue::new().with_date_time(date_time.format(&date_fmt)?),
            )]));

        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn get_status_inner(&self) -> Result<XfsMethodResponse> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetStatus);

        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)
    }

    pub(crate) fn park_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Park);

        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let _res = Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn get_capabilities_inner(&self) -> Result<Capabilities> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetCapabilities);

        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?.try_into()
    }

    pub(crate) fn set_capabilities_inner(&self, caps: &Capabilities) -> Result<()> {
        let call = XfsMethodCall::create(XfsMethodName::SetCapabilities, [XfsParam::from(caps)]);

        let timeout = std::time::Duration::from_millis(50);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let _res = Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_call(
        usb: &UsbDeviceHandle,
        call: &XfsMethodCall,
        timeout: std::time::Duration,
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
        timeout: std::time::Duration,
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
