//! System configuration types and functionality.

use crate::{capabilities::Capabilities, check_res, Result};

/// Sets the BNR [Capabilities].
///
/// Those settings are persistent over power cycles; please refer to [Capabilities] for more details about settable capabilities, and their default values.
///
/// Returns the set [Capabilities] on success, an error code otherwise.
pub fn set_capabilities(caps: &Capabilities) -> Result<Capabilities> {
    let mut sys_caps = bnr_sys::XfsCapabilities::from(caps);

    check_res(
        unsafe { bnr_sys::bnr_SetCapabilities(&mut sys_caps as *mut _) },
        "set_capabilities",
    )?;

    Ok(sys_caps.into())
}
