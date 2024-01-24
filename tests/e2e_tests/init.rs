use bnr::{init, Result};

use super::common;

#[test]
fn test_open() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;
    init::close()?;

    Ok(())
}

#[test]
fn test_cancel() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;
    init::cancel()?;
    init::close()?;

    Ok(())
}

#[test]
#[cfg(feature = "test-reboot")]
fn test_reboot() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reboot()?;

    Ok(())
}
