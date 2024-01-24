use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_get_status() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    let status = handle.get_status()?;

    log::debug!("Device status: {status}");

    Ok(())
}
