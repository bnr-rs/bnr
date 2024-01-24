use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_open() -> Result<()> {
    let _lock = common::init();

    let _handle = DeviceHandle::open(None, None, None)?;

    Ok(())
}

#[test]
fn test_reset() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.reset()?;

    Ok(())
}

#[test]
fn test_cancel() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.cancel()?;

    Ok(())
}

#[test]
fn test_close() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;
    handle.close()?;

    Ok(())
}
