use std::{thread, time};
use bnr::{cash, init, Result};

use super::common;

#[test]
fn test_cash_in_start() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::cash_in_start()?;

    init::close()?;

    Ok(())
}
