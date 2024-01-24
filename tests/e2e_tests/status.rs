use bnr::{init, status, Result};

use super::common;

#[test]
fn test_get_status() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let _status = status::get_status()?;

    init::close()?;

    Ok(())
}
