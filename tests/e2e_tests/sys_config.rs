use bnr::{self, init, sys_config, Result};

use super::common;

#[test]
fn test_set_capabilities() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let _caps = sys_config::set_capabilities(&bnr::Capabilities::new())?;

    init::close()?;

    Ok(())
}
