//! System configuration types and functionality.

use bnr_xfs::Capabilities;

use crate::{with_handle, Result};

/// Sets the BNR [Capabilities].
///
/// Those settings are persistent over power cycles; please refer to [Capabilities] for more details about settable capabilities, and their default values.
///
/// Returns the set [Capabilities] on success, an error code otherwise.
pub fn set_capabilities(caps: &Capabilities) -> Result<Capabilities> {
    with_handle::<Capabilities>(|h| {
        h.set_capabilities(caps)?;
        h.get_capabilities()
    })
}
