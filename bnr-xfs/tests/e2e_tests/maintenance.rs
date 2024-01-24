use bnr_xfs::{DeviceHandle, Result};

use super::common;

#[test]
fn test_park() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.park()
}
