use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use base64::Engine;
use datetime::format_description::well_known::{
    iso8601::{Config, TimePrecision},
    Iso8601,
};
use time as datetime;

use super::*;
use crate::callback_response::{
    CallbackIntermediateResponse, CallbackOperationResponse, CallbackStatusResponse,
};
use crate::cash_unit::TransportCount;
use crate::currency::{Currency, Denomination};
use crate::xfs::{
    self,
    method_call::{XfsMethodCall, XfsMethodName},
    method_response::{XfsMethodResponse, XfsMethodResponseStruct},
    params::{XfsParam, XfsParams},
    value::XfsValue,
    OperationId,
};

const INIT_COUNT: u64 = 1;
static CALL_COUNTER: AtomicU64 = AtomicU64::new(INIT_COUNT);
const TIMEOUT: u64 = 5000;
const CALLBACK_TIMEOUT: u64 = 0;

/// Class identifier of the `MainModule`.
pub const MAIN_MODULE_CLASS: i64 = 0xE0000;

static OP_COMPLETED_FN_NOP: OperationCompletedFn =
    |_id: i32, _op_id: i32, _res: i32, _ext_res: i32, _details: &mut dyn CallbackArg| {};
static INTERMEDIATE_OCCURRED_FN_NOP: IntermediateOccurredFn =
    |_id: i32, _op_id: i32, _reason: i32, _details: &mut dyn CallbackArg| {};
static STATUS_OCCURRED_FN_NOP: StatusOccurredFn =
    |_status: i32, _res: i32, _ext_res: i32, _details: &mut dyn CallbackArg| {};

pub(crate) fn call_counter() -> u64 {
    CALL_COUNTER.load(Ordering::Relaxed)
}

pub(crate) fn call_counter_base64() -> String {
    let count = call_counter();
    base64::engine::general_purpose::STANDARD.encode(format!("{count}").as_bytes())
}

pub(crate) fn set_call_counter(count: u64) {
    CALL_COUNTER.store(count, Ordering::SeqCst)
}

pub(crate) fn increment_call_counter() -> u64 {
    let count = call_counter().saturating_mul(2);
    set_call_counter(count);
    count
}

pub(crate) fn reset_call_counter() {
    CALL_COUNTER.store(INIT_COUNT, Ordering::SeqCst)
}

impl DeviceHandle {
    pub(crate) fn open_inner(
        usb: UsbDeviceHandle,
        op_completed_callback: Option<OperationCompletedFn>,
        intermediate_occurred_callback: Option<IntermediateOccurredFn>,
        status_occurred_callback: Option<StatusOccurredFn>,
    ) -> Result<Self> {
        let timeout = std::time::Duration::from_millis(TIMEOUT);

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

        let ret = Self {
            usb: Arc::new(usb),
            stop_listener: Arc::new(AtomicBool::new(false)),
            op_completed_callback,
            intermediate_occurred_callback,
            status_occurred_callback,
        };

        ret.start_background_listener(Arc::clone(&ret.stop_listener))?;

        let usb = ret.usb();

        let call = XfsMethodCall::create(
            XfsMethodName::GetIdentification,
            [XfsParam::create(
                XfsValue::new().with_int(MAIN_MODULE_CLASS),
            )],
        );

        // write the `getIdentification` call to the call endpoint
        Self::write_call(usb, &call, timeout)?;

        // read the response
        Self::read_response(usb, call.name()?, timeout).ok();

        Ok(ret)
    }

    pub(crate) fn find_usb() -> Result<UsbDeviceHandle> {
        let ctx = rusb::Context::new()?;
        let dev_list = rusb::DeviceList::new_with_context(ctx)?;

        let dev = dev_list.iter().find(|d| {
            if let Ok(desc) = d.device_descriptor() {
                desc.vendor_id() == BNR_VID && desc.product_id() == BNR_PID
            } else {
                false
            }
        });

        Ok(dev.ok_or(Error::Usb(format!("failed to find a USB device with the correct VID({BNR_VID:04x}):PID({BNR_PID:04x}) pair")))?.open()?)
    }

    pub(crate) fn start_background_listener(&self, stop: Arc<AtomicBool>) -> Result<()> {
        let usb = self.usb_clone();

        let op_complete = self.op_completed_callback().unwrap_or(OP_COMPLETED_FN_NOP);

        let intermediate_occurred = self
            .intermediate_occurred_callback()
            .unwrap_or(INTERMEDIATE_OCCURRED_FN_NOP);

        let status_occurred = self
            .status_occurred_callback()
            .unwrap_or(STATUS_OCCURRED_FN_NOP);

        std::thread::spawn(move || -> Result<()> {
            let timeout = std::time::Duration::from_millis(CALLBACK_TIMEOUT);

            while !stop.load(Ordering::Relaxed) {
                if let Ok(msg) = Self::read_callback_call(&usb, timeout) {
                    let res = match msg.name() {
                        Ok(XfsMethodName::OperationCompleteOccurred) => {
                            let mut params_iter =
                                msg.params().params().iter().map(|m| m.inner().value());

                            let id = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            let op_id: OperationId = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .try_into()?;

                            let ret = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            let stat = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            if let Some(xfs) = params_iter.next() {
                                match xfs.xfs_struct() {
                                    Some(xfs)
                                        if xfs.find_member(Currency::xfs_name()).is_ok()
                                            && xfs
                                                .find_member(Denomination::xfs_name())
                                                .is_ok() =>
                                    {
                                        let mut cash_order = CashOrder::try_from(xfs)?;
                                        op_complete(id, op_id.into(), ret, stat, &mut cash_order);
                                    }
                                    _ => op_complete(id, op_id.into(), ret, stat, &mut ()),
                                }
                            } else {
                                op_complete(id, op_id.into(), ret, stat, &mut ());
                            }

                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                CallbackOperationResponse::create(id, op_id.into()).into(),
                            )]))
                        }
                        Ok(XfsMethodName::IntermediateOccurred) => {
                            let mut params_iter =
                                msg.params().params().iter().map(|m| m.inner().value());

                            let id = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            let op_id: OperationId = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .try_into()?;

                            let ret = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            if let Some(xfs) = params_iter.next() {
                                match xfs.xfs_struct() {
                                    Some(xfs)
                                        if xfs.find_member(Currency::xfs_name()).is_ok()
                                            && xfs
                                                .find_member(Denomination::xfs_name())
                                                .is_ok() =>
                                    {
                                        let mut cash_order = CashOrder::try_from(xfs)?;
                                        intermediate_occurred(
                                            id,
                                            op_id.into(),
                                            ret,
                                            &mut cash_order,
                                        );
                                    }
                                    _ => intermediate_occurred(id, op_id.into(), ret, &mut ()),
                                }
                            } else {
                                intermediate_occurred(id, op_id.into(), ret, &mut ());
                            }

                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                CallbackIntermediateResponse::create(id, op_id.into()).into(),
                            )]))
                        }
                        Ok(XfsMethodName::StatusOccurred) => {
                            let mut params_iter =
                                msg.params().params().iter().map(|m| m.inner().value());

                            let id = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            let op_id: OperationId = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .try_into()?;

                            let ret = params_iter
                                .next()
                                .cloned()
                                .unwrap_or(XfsValue::new().with_i4(0))
                                .i4()
                                .cloned()
                                .unwrap_or(0);

                            if let Some(xfs) = params_iter.next() {
                                match xfs.xfs_struct() {
                                    Some(xfs)
                                        if xfs.find_member(Currency::xfs_name()).is_ok()
                                            && xfs
                                                .find_member(Denomination::xfs_name())
                                                .is_ok() =>
                                    {
                                        let mut cash_order = CashOrder::try_from(xfs)?;
                                        status_occurred(id, op_id.into(), ret, &mut cash_order);
                                    }
                                    _ => status_occurred(id, op_id.into(), ret, &mut ()),
                                }
                            } else {
                                status_occurred(id, op_id.into(), ret, &mut ());
                            }

                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                CallbackStatusResponse::create(id, ret).into(),
                            )]))
                        }
                        Err(_err) if msg.name_str().is_empty() => {
                            log::trace!("No callback call");
                            None
                        }
                        _stat => {
                            let name = msg.name_str();
                            log::warn!("Received unknown device-sent message, {name}: {msg:?}");
                            None
                        }
                    };

                    if let Some(res) = res.as_ref() {
                        Self::write_callback_response(&usb, res, msg.name()?, timeout)?;
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(256));
            }

            Ok(())
        });

        Ok(())
    }

    pub(crate) fn stop_background_listener(&self) {
        self.stop_listener.store(true, Ordering::SeqCst);
    }

    pub(crate) fn reset_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Reset)
            .with_params(XfsParams::create([count]));

        reset_call_counter();

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        match Self::read_response(usb, call.name()?, timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn cancel_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Cancel);
        let timeout = std::time::Duration::from_millis(TIMEOUT);
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
        let timeout = std::time::Duration::from_millis(TIMEOUT);
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
        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        reset_call_counter();

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
        let timeout = std::time::Duration::from_millis(TIMEOUT);
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

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn get_status_inner(&self) -> Result<CdrStatus> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetStatus);

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?.try_into()
    }

    pub(crate) fn park_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Park);

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let _res = Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn get_capabilities_inner(&self) -> Result<Capabilities> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetCapabilities);

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?.try_into()
    }

    pub(crate) fn set_capabilities_inner(&self, caps: &Capabilities) -> Result<Capabilities> {
        let call = XfsMethodCall::create(XfsMethodName::SetCapabilities, [XfsParam::from(caps)]);

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        let res = Self::read_response(usb, call.name()?, timeout)?;

        match Capabilities::try_from(&res) {
            Ok(c) => Ok(c),
            Err(_err) => Ok(*caps),
        }
    }

    pub(crate) fn cash_in_start_inner(&self) -> Result<()> {
        increment_call_counter();

        let name = XfsMethodName::CashInStart;
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(name)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn cash_in_inner(
        &self,
        limit: Option<u32>,
        currency: Option<CurrencyCode>,
    ) -> Result<()> {
        increment_call_counter();

        let name = XfsMethodName::CashIn;
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = match (limit, currency) {
            (None, None) => XfsMethodCall::new()
                .with_name(name)
                .with_params(XfsParams::create([count])),
            (Some(l), None) => XfsMethodCall::new()
                .with_name(name)
                .with_params(XfsParams::create([
                    XfsParam::create(XfsValue::new().with_int(l as i64)),
                    count,
                ])),
            (None, Some(c)) => XfsMethodCall::new()
                .with_name(name)
                .with_params(XfsParams::create([
                    XfsParam::create(XfsValue::new().with_string(<&str>::from(c))),
                    count,
                ])),
            (Some(l), Some(c)) => {
                XfsMethodCall::new()
                    .with_name(name)
                    .with_params(XfsParams::create([
                        XfsParam::create(XfsValue::new().with_int(l as i64)),
                        XfsParam::create(XfsValue::new().with_string(<&str>::from(c))),
                        count,
                    ]))
            }
        };

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn cash_in_end_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CashInEnd)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn cash_in_rollback_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CashInRollback)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn eject_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Eject)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn empty_inner(&self, pcu_name: &str, to_float: bool) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Empty)
            .with_params(XfsParams::create([
                XfsParam::create(XfsValue::new().with_string(pcu_name)),
                XfsParam::create(XfsValue::new().with_boolean(to_float as u8)),
                count,
            ]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn present_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Present)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn cancel_waiting_cash_taken_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CancelWaitingCashTaken)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn retract_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Retract)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn query_cash_unit_inner(&self) -> Result<CashUnit> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::QueryCashUnit)
            .with_params(XfsParams::create([count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?.try_into()
    }

    pub(crate) fn configure_cash_unit_inner(
        &self,
        transport_count: u32,
        lcu_list: &LogicalCashUnitList,
        pcu_list: &PhysicalCashUnitList,
    ) -> Result<()> {
        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::ConfigureCashUnit)
            .with_params(XfsParams::create([
                XfsParam::create(TransportCount::create(transport_count).into()),
                XfsParam::create(lcu_list.into()),
                XfsParam::create(pcu_list.into()),
            ]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn update_cash_unit_inner(
        &self,
        transport_count: u32,
        lcu_list: &LogicalCashUnitList,
        pcu_list: &PhysicalCashUnitList,
    ) -> Result<()> {
        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::UpdateCashUnit)
            .with_params(XfsParams::create([
                XfsParam::create(TransportCount::create(transport_count).into()),
                XfsParam::create(lcu_list.into()),
                XfsParam::create(pcu_list.into()),
            ]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn denominate_inner(&self, request: &DispenseRequest) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Denominate)
            .with_params(XfsParams::create([XfsParam::create(request.into()), count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

        Ok(())
    }

    pub(crate) fn dispense_inner(&self, request: &DispenseRequest) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Dispense)
            .with_params(XfsParams::create([XfsParam::create(request.into()), count]));

        let timeout = std::time::Duration::from_millis(TIMEOUT);
        let usb = self.usb();

        Self::write_call(usb, &call, timeout)?;

        Self::read_response(usb, call.name()?, timeout)?;

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
                Err(err)
            }
        }
    }

    /// Reads an XFS method response (as a string) from the BNR response endpoint.
    pub fn read_callback_call(
        usb: &UsbDeviceHandle,
        timeout: std::time::Duration,
    ) -> Result<XfsMethodCall> {
        // Responses can be very large, so read from the endpoint in 4K chunks.
        let mut res_buf = [0u8; 4096];
        let mut res_acc = Vec::with_capacity(4096);

        let mut read = match usb.read_bulk(BNR_CALLBACK_CALL_EP, &mut res_buf[..], timeout) {
            Ok(r) => r,
            Err(_err) => {
                log::trace!("No callback call");

                return Ok(XfsMethodCall::new());
            }
        };

        res_acc.extend_from_slice(&res_buf[..read]);
        while read == res_buf.len() {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf.copy_from_slice([0u8; 4096].as_ref());
            read = match usb.read_bulk(BNR_CALLBACK_CALL_EP, &mut res_buf[..], timeout) {
                Ok(r) => r,
                Err(err) => {
                    let err_msg = format!("Error reading callback call: {err}");
                    log::warn!("{err_msg}");

                    0
                }
            };
            res_acc.extend_from_slice(&res_buf[..read]);
        }

        let call_str = std::str::from_utf8(res_acc.as_ref()).unwrap_or("");
        log::trace!("Raw callback call: {call_str}");

        xfs::from_str::<XfsMethodCall>(call_str)
    }

    /// Writes an [XfsMethodCall] to the BNR device.
    pub fn write_callback_response(
        usb: &UsbDeviceHandle,
        res: &XfsMethodResponse,
        name: XfsMethodName,
        timeout: std::time::Duration,
    ) -> Result<()> {
        match usb.write_bulk(
            BNR_CALLBACK_RESPONSE_EP,
            xfs::to_iso_string(res)?.as_bytes(),
            timeout,
        ) {
            Ok(_) => Ok(()),
            Err(err) => {
                let err_msg = format!("Error writing {name} callback response message: {err}");
                log::warn!("{err_msg}");
                Err(err.into())
            }
        }
    }
}
