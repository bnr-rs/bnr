use bnr_xfs::{DeviceHandle, Result};

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
