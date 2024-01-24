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
