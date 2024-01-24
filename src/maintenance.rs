//! Types and functionality for performing maintenance on BNR devices.

use crate::{check_res, Result};

/// Prepares all modules in the BNR device to be removed.
pub fn park() -> Result<()> {
    check_res(unsafe { bnr_sys::bnr_Park() }, "park")
}
