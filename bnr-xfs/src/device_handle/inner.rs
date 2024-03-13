use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};

use base64::Engine;
use datetime::format_description::well_known::iso8601::{Config, TimePrecision};
use datetime::format_description::well_known::Iso8601;
use time as datetime;

use super::usb::UsbDeviceHandle;
use super::*;
use crate::currency::{Currency, Denomination};
use crate::xfs::method_call::{XfsMethodCall, XfsMethodName};
use crate::xfs::method_response::XfsMethodResponse;
use crate::xfs::params::{XfsParam, XfsParams};
use crate::xfs::value::XfsValue;
use crate::TransportCount;
use crate::{CallbackIntermediateResponse, CallbackOperationResponse, CallbackStatusResponse};

const INIT_COUNT: u64 = 1;
static CALL_COUNTER: AtomicU64 = AtomicU64::new(INIT_COUNT);

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
        let (response_tx, response_rx) = mpsc::channel();

        let ret = Self {
            usb: Arc::new(usb),
            stop_listener: Arc::new(AtomicBool::new(false)),
            op_completed_callback,
            intermediate_occurred_callback,
            status_occurred_callback,
            response_rx,
        };

        ret.start_background_listener(response_tx, Arc::clone(&ret.stop_listener))?;

        let usb = ret.usb();

        let call = XfsMethodCall::create(
            XfsMethodName::GetIdentification,
            [XfsParam::create(
                XfsValue::new().with_int(MAIN_MODULE_CLASS),
            )],
        );

        // write the `getIdentification` call to the call endpoint
        usb.write_call(&call)?;

        // read the response
        usb.read_response(call.name()?).ok();

        Ok(ret)
    }

    pub(crate) fn start_background_listener(
        &self,
        response_tx: mpsc::Sender<XfsMethodCall>,
        stop: Arc<AtomicBool>,
    ) -> Result<()> {
        let usb = self.usb_clone();

        let op_complete = self.op_completed_callback().unwrap_or(OP_COMPLETED_FN_NOP);

        let intermediate_occurred = self
            .intermediate_occurred_callback()
            .unwrap_or(INTERMEDIATE_OCCURRED_FN_NOP);

        let status_occurred = self
            .status_occurred_callback()
            .unwrap_or(STATUS_OCCURRED_FN_NOP);

        std::thread::spawn(move || -> Result<()> {
            while !stop.load(Ordering::Relaxed) {
                if let Ok(msg) = usb.read_callback_call() {
                    log::trace!("Callback call: {msg}");
                    let res_id = msg.call_id().unwrap_or(-1);
                    let mut callback_arg = match msg.xfs_struct() {
                        Ok(xfs)
                            if xfs.find_member(Currency::xfs_name()).is_ok()
                                && xfs.find_member(Denomination::xfs_name()).is_ok() =>
                        {
                            CashOrder::try_from(xfs)
                                .map_err(|err| log::error!("Error converting CashOrder: {err}"))
                                .ok()
                        }
                        _ => None,
                    };
                    let msg_name = msg.name()?;
                    let op_id: i32 = msg.operation_id()?.into();
                    let result = msg.result().unwrap_or(0);
                    let ext_result = msg.ext_result().unwrap_or(0);
                    log::trace!("Callback message name: {msg_name}");
                    let xfs_res = match msg_name {
                        XfsMethodName::OperationCompleteOccurred => {
                            log::trace!("OperationComplete occurred: {msg}");
                            if let Some(cash_order) = callback_arg.as_mut() {
                                op_complete(res_id, op_id, result, ext_result, cash_order);
                            } else {
                                op_complete(res_id, op_id, result, ext_result, &mut ());
                            }
                            response_tx.send(msg)?;

                            let response = CallbackOperationResponse::create(op_id, res_id);
                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                XfsValue::new().with_xfs_struct(response.into()),
                            )]))
                        }
                        XfsMethodName::IntermediateOccurred => {
                            log::trace!("Intermediate occurred: {msg}");
                            if let Some(cash_order) = callback_arg.as_mut() {
                                intermediate_occurred(res_id, op_id, result, cash_order);
                            } else {
                                intermediate_occurred(res_id, op_id, result, &mut ());
                            }

                            let response = CallbackIntermediateResponse::create(op_id, res_id);
                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                XfsValue::new().with_xfs_struct(response.into()),
                            )]))
                        }
                        XfsMethodName::StatusOccurred => {
                            log::trace!("Status occurred: {msg}");
                            if let Some(cash_order) = callback_arg.as_mut() {
                                status_occurred(res_id, op_id, result, cash_order);
                            } else {
                                status_occurred(res_id, op_id, result, &mut ());
                            }
                            let response = CallbackStatusResponse::create(res_id, result);
                            Some(XfsMethodResponse::new_params([XfsParam::create(
                                XfsValue::new().with_xfs_struct(response.into()),
                            )]))
                        }
                        _ => None,
                    };
                    if let Some(res) = xfs_res {
                        usb.write_callback_response(&res, msg_name)?;
                    }
                }
            }

            Ok(())
        });

        Ok(())
    }

    pub(crate) fn stop_background_listener(&self) {
        self.stop_listener.store(true, Ordering::SeqCst);
    }

    pub(crate) fn handle_async_call(&self, call_id: i32) -> Result<()> {
        let mut res_call: Option<XfsMethodCall> = None;
        let response_timeout = std::time::Duration::from_millis(4250);
        let now = std::time::SystemTime::now();

        while res_call.is_none() && now.elapsed()? < response_timeout {
            if let Ok(msg) = self.response_rx.try_recv() {
                let res_id = msg.call_id().unwrap_or(-1);
                if res_id != -1 && res_id == call_id {
                    res_call = Some(msg);
                }
            }
        }

        if let Some(msg) = res_call {
            log::debug!("async response: {msg}");
            let result = msg.result().unwrap_or(-1);
            match result {
                0 => Ok(()),
                -1 => {
                    let err_msg = format!("async response: missing event result: {msg}");
                    log::error!("{err_msg}");
                    Err(Error::Xfs(err_msg))
                }
                _ => {
                    let err_msg = format!("async response: call failed: {msg}");
                    log::error!("{err_msg}");
                    Err(Error::Xfs(err_msg))
                }
            }
        } else {
            Err(Error::Xfs(
                "async response: no operation complete response".into(),
            ))
        }
    }

    pub(crate) fn reset_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Reset)
            .with_params(XfsParams::create([count]));

        reset_call_counter();

        let usb = self.usb();

        usb.write_call(&call)?;

        match usb.read_response(call.name()?) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn cancel_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Cancel);

        let usb = self.usb();
        usb.write_call(&call)?;

        match usb.read_response(call.name()?) {
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
        let usb = self.usb();

        usb.write_call(&call)?;

        match usb.read_response(call.name()?) {
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
        let usb = self.usb();

        usb.write_call(&call)?;

        reset_call_counter();

        match usb.read_response(call.name()?) {
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

        let usb = self.usb();
        usb.write_call(&call)?;

        let res = usb.read_response(call.name()?)?;
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

        let usb = self.usb();
        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn get_status_inner(&self) -> Result<CdrStatus> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetStatus);

        let usb = self.usb();
        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn park_inner(&self) -> Result<()> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::Park);

        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn get_capabilities_inner(&self) -> Result<Capabilities> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::GetCapabilities);

        let usb = self.usb();
        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn set_capabilities_inner(&self, caps: &Capabilities) -> Result<Capabilities> {
        let call = XfsMethodCall::create(XfsMethodName::SetCapabilities, [XfsParam::from(caps)]);

        let usb = self.usb();

        usb.write_call(&call)?;
        let res = usb.read_response(call.name()?)?;

        match Capabilities::try_from(&res) {
            Ok(c) => Ok(c),
            Err(_err) => Ok(*caps),
        }
    }

    pub(crate) fn cash_in_start_inner(&self) -> Result<()> {
        let name = XfsMethodName::CashInStart;
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(name)
            .with_params(XfsParams::create([count]));

        let call_id = {
            let usb = self.usb();
            usb.write_call(&call)?;
            usb.read_response(call.name()?)?.call_id()?
        };

        self.handle_async_call(call_id)?;

        set_call_counter(call_id as u64);

        Ok(())
    }

    pub(crate) fn cash_in_inner(
        &self,
        limit: Option<u32>,
        currency: Option<CurrencyCode>,
    ) -> Result<()> {
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

        let call_id = {
            let usb = self.usb();
            usb.write_call(&call)?;
            usb.read_response(call.name()?)?.call_id()?
        };

        set_call_counter(call_id as u64);

        Ok(())
    }

    pub(crate) fn cash_in_end_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CashInEnd)
            .with_params(XfsParams::create([count]));

        let call_id = {
            let usb = self.usb();
            usb.write_call(&call)?;
            usb.read_response(call.name()?)?.call_id()?
        };

        self.handle_async_call(call_id)?;

        set_call_counter(call_id as u64);

        Ok(())
    }

    pub(crate) fn cash_in_rollback_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CashInRollback)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();
        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn eject_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Eject)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();
        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

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

        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn present_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Present)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn cancel_waiting_cash_taken_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::CancelWaitingCashTaken)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn retract_inner(&self) -> Result<()> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::Retract)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();

        usb.write_call(&call)?;

        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn query_cash_unit_inner(&self) -> Result<CashUnit> {
        increment_call_counter();
        let count = XfsParam::create(XfsValue::new().with_base64(call_counter_base64()));

        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::QueryCashUnit)
            .with_params(XfsParams::create([count]));

        let usb = self.usb();

        usb.write_call(&call)?;

        usb.read_response(call.name()?)?.try_into()
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

        let usb = self.usb();

        usb.write_call(&call)?;

        usb.read_response(call.name()?)?;

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

        let usb = self.usb();

        usb.write_call(&call)?;

        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn denominate_inner(&self, request: &DispenseRequest) -> Result<()> {
        let call = XfsMethodCall::from(request).with_name(XfsMethodName::Denominate);

        let call_id = {
            let usb = self.usb();
            usb.write_call(&call)?;
            usb.read_response(call.name()?)?.call_id()?
        };

        self.handle_async_call(call_id)
    }

    pub(crate) fn dispense_inner(&self, request: &DispenseRequest) -> Result<()> {
        let call = XfsMethodCall::from(request).with_name(XfsMethodName::Dispense);

        let call_id = {
            let usb = self.usb();

            usb.write_call(&call)?;
            usb.read_response(call.name()?)?.call_id()?
        };

        self.handle_async_call(call_id)
    }

    pub(crate) fn stop_session_inner(&self) -> Result<()> {
        let call = XfsMethodCall::create(XfsMethodName::StopSession, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn query_denominations_inner(&self) -> Result<DenominationList> {
        let call = XfsMethodCall::new().with_name(XfsMethodName::QueryDenominations);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn update_denominations_inner(&self, request: &DenominationList) -> Result<()> {
        let call = XfsMethodCall::new()
            .with_name(XfsMethodName::UpdateDenominations)
            .with_params(XfsParams::create([XfsParam::create(request.into())]));
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?;

        Ok(())
    }

    pub(crate) fn query_billset_ids_inner(&self) -> Result<BillsetIdList> {
        let call = XfsMethodCall::create(XfsMethodName::QueryBillsetIds, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn get_bill_acceptance_history_inner(&self) -> Result<BillAcceptanceHistory> {
        let call = XfsMethodCall::create(XfsMethodName::GetBillAcceptanceHistory, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn get_bill_dispense_history_inner(&self) -> Result<BillDispenseHistory> {
        let call = XfsMethodCall::create(XfsMethodName::GetBillDispenseHistory, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn get_failure_history_inner(&self) -> Result<SystemFailureHistory> {
        let call = XfsMethodCall::create(XfsMethodName::GetFailureHistory, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn get_restart_history_inner(&self) -> Result<SystemRestartHistory> {
        let call = XfsMethodCall::create(XfsMethodName::GetRestartHistory, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }

    pub(crate) fn get_use_history_inner(&self) -> Result<SystemUseHistory> {
        let call = XfsMethodCall::create(XfsMethodName::GetUseHistory, []);
        let usb = self.usb();

        usb.write_call(&call)?;
        usb.read_response(call.name()?)?.try_into()
    }
}
