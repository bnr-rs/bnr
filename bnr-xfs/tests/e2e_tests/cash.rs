use bnr_xfs::{DeviceHandle, DispenseRequest, Result};

use super::common;

#[test]
fn test_cash_in_start() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.cash_in_start()?;
    handle.cancel()?;
    handle.cash_in_end()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_cash_in() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.cash_in_start()?;

    handle.cash_in(None, None)?;

    handle.cancel()?;
    handle.cash_in_end()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_cash_in_end() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.cash_in_start()?;
    handle.cancel()?;
    handle.cash_in_end()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_cash_in_rollback() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.cash_in_start()?;
    handle.cash_in(None, None)?;
    handle.cancel()?;
    handle.cash_in_rollback()?;
    handle.cash_in_end()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_eject() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.eject()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_empty() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.empty("pcu0", false)?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_present() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.present()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_cancel_waiting_cash_taken() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.cancel_waiting_cash_taken()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_retract() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    handle.retract()?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_query_cash_unit() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let cu = handle.query_cash_unit()?;

    log::debug!("CashUnit: {cu}");

    handle.reset()?;

    Ok(())
}

#[test]
fn test_update_cash_unit() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let cu = handle.query_cash_unit()?;

    log::debug!("CashUnit: {cu}");

    // Not realistic, this is essentially a no-op.
    // We're just updating with the same information already on the device.
    //
    // This is only used to exercise the API.
    handle.update_cash_unit(
        cu.transport_count(),
        cu.logical_cash_unit_list(),
        cu.physical_cash_unit_list(),
    )?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_denominate() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    // Not realistic, this is essentially a no-op.
    //
    // This is only used to exercise the API.
    handle.denominate(&DispenseRequest::new())?;

    handle.reset()?;

    Ok(())
}

#[test]
fn test_dispense() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    // Not realistic, this is essentially a no-op.
    //
    // This is only used to exercise the API.
    handle.dispense(&DispenseRequest::new())?;

    handle.reset()?;

    Ok(())
}
