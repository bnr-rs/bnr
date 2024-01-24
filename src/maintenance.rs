//! Types and functionality for performing maintenance on BNR devices.

use crate::{with_handle, Result};

/// Prepares all modules in the BNR device to be removed.
pub fn park() -> Result<()> {
    with_handle::<()>(|h| h.park())
}
