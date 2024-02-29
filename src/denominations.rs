//! Functions for denomination operations.

use bnr_xfs::{BillsetIdList, DenominationList};

use crate::{with_handle, Result};

/// Gets a list of denominations in the BNR.
///
/// Returns:
///
/// - Ok([DenominationList]): list of the denominations currently defined in the BNR.
/// - Error conditions: see [update_denominations] for a list of error code descriptions.
pub fn query_denominations() -> Result<DenominationList> {
    with_handle::<DenominationList>(|h| h.query_denominations())
}

/// Updates the settings for a list of denominations.
///
/// For each [DenominationInfo](bnr_xfs::denominations::DenominationInfo) element of the [DenominationList](bnr_xfs::denominations::DenominationList),
/// the application can update its validation settings.
///
/// From the BNR API docs:
///
/// ```no_build,no_run
/// Those settings are persistent over power cycles; please refer to DenominationInfo for more details about settable properties, and their default values.
///
/// @param[in] DenominationList This list of denominations will be a modified version of the one obtained from query_denominations() call.
/// ```
///
/// Returns:
///
/// - Ok(()) on success
/// - Error conditions:
///   - `#XFS_E_ILLEGAL` - A dispense command is already active on the BNR.
///   - `#XFS_E_NOT_SUPPORTED` - operation not supported by the BNR firmware version.
///   - `#XFS_E_PARAMETER_INVALID` - Invalid array size. The array size is bigger than expected.
///   - `#XFS_E_CDR_CASHIN_ACTIVE` - A cashIn command has been issued and is already active.
///   - `#XFS_E_FAILURE` - a command is already running on the BNR or an internal error occured.
pub fn update_denominations(request: &DenominationList) -> Result<()> {
    with_handle::<()>(|h| h.update_denominations(request))
}

/// Queries the device for the configured [BillsetIdList].
///
/// **NOTE** Firmware Compatibility: This function requires a BNR FW v1.12.0 or newer. With older FW versions, the return will be #XFS_E_NOT_SUPPORTED.
pub fn query_billset_ids() -> Result<BillsetIdList> {
    with_handle::<BillsetIdList>(|h| h.query_billset_ids())
}
