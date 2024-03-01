use crate::create_xfs_struct;
use crate::xfs::method_response::XfsMethodResponse;
use crate::{Error, Result};

use super::{CashTypeRecycleHistoryItems, InletAcceptanceHistory, LoaderAcceptanceHistory};

create_xfs_struct!(
    BillAcceptanceHistory,
    "billAcceptanceHistory",
    [
        inlet_acceptance_history: InletAcceptanceHistory,
        loader_acceptance_history: LoaderAcceptanceHistory,
        recycle_acceptance_history: CashTypeRecycleHistoryItems
    ],
    "Represents the bill acceptance history."
);

impl TryFrom<&XfsMethodResponse> for BillAcceptanceHistory {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected BillAcceptanceHistory XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for BillAcceptanceHistory {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
