use bnr_xfs::{DenominationInfo, DenominationList, DeviceHandle, Result};

use super::common;

#[test]
fn test_update_denominations() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let mut denom_list = DenominationList::new();
    denom_list.push(DenominationInfo::new());

    handle.update_denominations(&denom_list)
}

#[test]
fn test_query_denominations() -> Result<()> {
    let _lock = common::init();

    let handle = DeviceHandle::open(None, None, None)?;

    handle.close()?;

    let denom_list = handle.query_denominations()?;

    log::debug!("BNR denomination list: {denom_list}");

    Ok(())
}
