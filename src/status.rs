//! Types and functions for handling system status events.

use bnr_xfs::CdrStatus;

use crate::{with_handle, Result};

/// Gets the status of the CDR device.
pub fn get_status() -> Result<CdrStatus> {
    with_handle::<CdrStatus>(|h| h.get_status())
}
