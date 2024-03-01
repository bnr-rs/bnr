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
