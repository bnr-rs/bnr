use bnr::{cash, init, Result};
use std::{thread, time};

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

#[test]
fn test_cash_in() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::cash_in_start()?;
    cash::cash_in(None, None)?;

    init::close()?;

    Ok(())
}

#[test]
fn test_cash_in_rollback() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::cash_in_start()?;
    cash::cash_in_rollback()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_cash_in_end() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::cash_in_start()?;
    cash::cash_in_end()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_eject() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::eject()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_empty() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::empty(&[0i8; 5], false)?;

    init::close()?;

    Ok(())
}

#[test]
fn test_query_cash_unit() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::query_cash_unit()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_configure_cash_unit() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::configure_cash_unit(
        0,
        &cash::LogicalCashUnitList::new(),
        &cash::PhysicalCashUnitList::new(),
    )?;

    init::close()?;

    Ok(())
}

#[test]
fn test_update_cash_unit() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::update_cash_unit(
        0,
        &cash::LogicalCashUnitList::new()
            .with_size(1u32)
            .with_items([cash::LogicalCashUnit::new(); cash::LCU_LIST_LEN]),
        &cash::PhysicalCashUnitList::new()
            .with_size(1u32)
            .with_items([cash::PhysicalCashUnit::new(); cash::PCU_LIST_LEN]),
    )?;

    init::close()?;

    Ok(())
}

#[test]
fn test_reset_cash_unit_counts() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::reset_cash_unit_counts()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_denominate() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::denominate(&cash::DispenseRequest::new())?;

    init::close()?;

    Ok(())
}

#[test]
fn test_dispense() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::dispense(&cash::DispenseRequest::new())?;

    init::close()?;

    Ok(())
}

#[test]
fn test_present() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::present()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_cancel_waiting_cash_taken() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::cancel_waiting_cash_taken()?;

    init::close()?;

    Ok(())
}

#[test]
fn test_retract() -> Result<()> {
    let _lock = common::init();

    init::open(None, None, None)?;
    init::reset()?;

    thread::sleep(time::Duration::from_secs(5));

    cash::retract()?;

    init::close()?;

    Ok(())
}
