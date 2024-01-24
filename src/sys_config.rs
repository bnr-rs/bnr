//! System configuration types and functionality.

use time::OffsetDateTime;

use bnr_xfs::Capabilities;

use crate::{with_handle, Result};

/// Sets the BNR [Capabilities].
///
/// Those settings are persistent over power cycles; please refer to [Capabilities] for more details about settable capabilities, and their default values.
///
/// Returns the set [Capabilities] on success, an error code otherwise.
pub fn set_capabilities(caps: &Capabilities) -> Result<Capabilities> {
    with_handle::<Capabilities>(|h| h.set_capabilities(caps))
}

/// Gets the ISO 8601 formatted date-time from the device.
pub fn get_date_time() -> Result<OffsetDateTime> {
    with_handle::<OffsetDateTime>(|h| h.get_date_time())
}

/// Sets the ISO 8601 formatted date-time on the device to the provided time.
///
/// **NOTE** This setting is not persistent across reboots/power-cycles.
///
/// The default device time will reset to `2001-01-01 00:00:00`.
pub fn set_date_time(date_time: OffsetDateTime) -> Result<()> {
    with_handle::<()>(|h| h.set_date_time(date_time))
}

/// Sets the ISO 8601 formatted date-time on the device to the current time.
///
/// **NOTE** This setting is not persistent across reboots/power-cycles.
///
/// The default device time will reset to `2001-01-01 00:00:00`.
pub fn set_current_date_time() -> Result<()> {
    with_handle::<()>(|h| h.set_current_date_time())
}
