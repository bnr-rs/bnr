use bnr_xfs::device_handle::usb::UsbDeviceHandle;
use bnr_xfs::Result;

use super::common;

#[test]
fn test_usb_setup() -> Result<()> {
    let _lock = common::init();

    let _usb = UsbDeviceHandle::find_usb()?;

    log::info!("XFS USB device setup successful");

    Ok(())
}
