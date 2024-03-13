use bnr::{self, history, init, Result};

use super::common;

#[test]
fn test_get_bill_acceptance_history() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let history = history::get_bill_acceptance_history()?;
    log::debug!("Bill acceptance history: {history}");

    init::close()?;

    Ok(())
}

#[test]
fn test_get_bill_dispense_history() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let history = history::get_bill_dispense_history()?;
    log::debug!("Bill dispense history: {history}");

    init::close()?;

    Ok(())
}

#[test]
fn test_get_failure_history() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let history = history::get_failure_history()?;
    log::debug!("System failure history: {history}");

    init::close()?;

    Ok(())
}

#[test]
fn test_get_restart_history() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let history = history::get_restart_history()?;
    log::debug!("System restart history: {history}");

    init::close()?;

    Ok(())
}

#[test]
fn test_get_use_history() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    let history = history::get_use_history()?;
    log::debug!("System use history: {history}");

    init::close()?;

    Ok(())
}
