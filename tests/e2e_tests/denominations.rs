use bnr::{self, denominations, init, Result};
use std::{thread, time};

use crate::e2e_tests::common;

#[test]
fn test_query_denominations() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(1));

    let list = denominations::query_denominations()?;
    log::debug!("Denominations list: {list}");

    init::close()?;

    Ok(())
}

#[test]
fn test_update_denominations() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(1));

    let list = denominations::query_denominations()?;
    log::debug!("Denominations list: {list}");
    denominations::update_denominations(&list)?;

    init::close()?;

    Ok(())
}

#[test]
fn test_query_billset_ids() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(1));

    let ids = denominations::query_billset_ids()?;
    log::debug!("Billset IDs: {ids}");

    init::close()?;

    Ok(())
}
