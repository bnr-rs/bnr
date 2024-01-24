use bnr::{init, maintenance, Result};

use super::common;

#[test]
fn test_park() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let _status = maintenance::park()?;

    init::close()?;

    Ok(())
}
