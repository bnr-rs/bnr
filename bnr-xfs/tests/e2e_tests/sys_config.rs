use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_get_capabilities() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    let caps = handle.get_capabilities()?;

    log::debug!("Device capabilities: {caps}");

    Ok(())
}

#[test]
fn test_set_capabilities() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    let caps = handle.get_capabilities()?;

    log::debug!("Device capabilities: {caps}");

    handle.set_capabilities(&caps)?;

    assert_eq!(handle.get_capabilities()?, caps);

    Ok(())
}
