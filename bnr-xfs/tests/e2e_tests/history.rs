use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_get_bill_acceptance_history() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let history = handle.get_bill_acceptance_history()?;

    log::debug!("Bill acceptance history: {history}");

    Ok(())
}

#[test]
fn test_get_bill_dispense_history() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let history = handle.get_bill_dispense_history()?;

    log::debug!("Bill dispense history: {history}");

    Ok(())
}

#[test]
fn test_get_failure_history() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let history = handle.get_failure_history()?;

    log::debug!("System failure history: {history}");

    Ok(())
}

#[test]
fn test_get_restart_history() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let history = handle.get_restart_history()?;

    log::debug!("System restart history: {history}");

    Ok(())
}

#[test]
fn test_get_use_history() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;
    if date.year() == 2001 {
        handle.set_current_date_time()?;
    }

    let history = handle.get_use_history()?;

    log::debug!("System use history: {history}");
    log::debug!("System current time: {}", time::OffsetDateTime::try_from(history.current_date_time())?);

    Ok(())
}
