use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_open() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.close()?;

    Ok(())
}

#[test]
fn test_reset() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.close()?;

    handle.get_date_time()?;
    handle.reset()?;

    handle.close()?;

    Ok(())
}

#[test]
fn test_cancel() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.cancel()?;

    handle.close()?;

    Ok(())
}

#[test]
fn test_close() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.close()?;

    Ok(())
}

#[test]
#[cfg(feature = "test-reboot")]
fn test_reboot() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.reboot()?;

    Ok(())
}

#[test]
fn test_get_date_time() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;

    log::debug!("DateTime: {date}");

    handle.close()?;

    Ok(())
}

#[test]
fn test_set_date_time() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let date = handle.get_date_time()?;

    log::debug!("DateTime: {date}");

    handle.set_current_date_time()?;

    handle.close()?;

    Ok(())
}
