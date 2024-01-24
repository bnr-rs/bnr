use std::fmt;

use crate::xfs::method_response::XfsMethodResponse;
use crate::{impl_xfs_struct, Error, Result};

mod count;
mod counters;
mod lock;
mod logical_cash_unit;
mod number;
mod pcu_name;
mod physical_cash_unit;
mod size;
mod status;
mod threshold;
mod unit_id;

pub use count::*;
pub use counters::*;
pub use lock::*;
pub use logical_cash_unit::*;
pub use number::*;
pub use pcu_name::*;
pub use physical_cash_unit::*;
pub use size::*;
pub use status::*;
pub use threshold::*;
pub use unit_id::*;

/// Represents a cash unit in a BNR device.
///
/// Describes the entire set of [LogicalCashUnit]s and [PhysicalCashUnit]s present on a device.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashUnit {
    transport_count: TransportCount,
    logical_cash_unit_list: LogicalCashUnitList,
    physical_cash_unit_list: PhysicalCashUnitList,
}

impl CashUnit {
    /// Creates a new [CashUnit].
    pub fn new() -> Self {
        Self {
            transport_count: TransportCount::new(),
            logical_cash_unit_list: LogicalCashUnitList::new(),
            physical_cash_unit_list: PhysicalCashUnitList::new(),
        }
    }

    /// Gets the [TransportCount].
    pub const fn transport_count(&self) -> u32 {
        self.transport_count.inner()
    }

    /// Sets the [TransportCount].
    pub fn set_transport_count(&mut self, count: u32) {
        self.transport_count.set_inner(count);
    }

    /// Builder function that sets the [TransportCount].
    pub fn with_transport_count(mut self, count: u32) -> Self {
        self.set_transport_count(count);
        self
    }

    /// Gets a reference to the [LogicalCashUnitList].
    pub const fn logical_cash_unit_list(&self) -> &LogicalCashUnitList {
        &self.logical_cash_unit_list
    }

    /// Gets a mutable reference to the [LogicalCashUnitList].
    pub fn logical_cash_unit_list_mut(&mut self) -> &mut LogicalCashUnitList {
        &mut self.logical_cash_unit_list
    }

    /// Sets the [LogicalCashUnitList].
    pub fn set_logical_cash_unit_list(&mut self, lcu: LogicalCashUnitList) {
        self.logical_cash_unit_list = lcu;
    }

    /// Builder function that sets the [LogicalCashUnitList].
    pub fn with_logical_cash_unit_list(mut self, lcu: LogicalCashUnitList) -> Self {
        self.set_logical_cash_unit_list(lcu);
        self
    }

    /// Gets a reference to the [PhysicalCashUnitList].
    pub const fn physical_cash_unit_list(&self) -> &PhysicalCashUnitList {
        &self.physical_cash_unit_list
    }

    /// Sets the [PhysicalCashUnitList].
    pub fn set_physical_cash_unit_list(&mut self, pcu: PhysicalCashUnitList) {
        self.physical_cash_unit_list = pcu;
    }

    /// Builder function that sets the [PhysicalCashUnitList].
    pub fn with_physical_cash_unit_list(mut self, pcu: PhysicalCashUnitList) -> Self {
        self.set_physical_cash_unit_list(pcu);
        self
    }
}

impl fmt::Display for CashUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""transport_count":{},"#, self.transport_count)?;
        write!(
            f,
            r#""logical_cash_unit_list":{},"#,
            self.logical_cash_unit_list
        )?;
        write!(
            f,
            r#""physical_cash_unit_list":{}"#,
            self.physical_cash_unit_list
        )?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    CashUnit,
    "cashUnit",
    [
        transport_count: TransportCount,
        logical_cash_unit_list: LogicalCashUnitList,
        physical_cash_unit_list: PhysicalCashUnitList
    ]
);

impl TryFrom<&XfsMethodResponse> for CashUnit {
    type Error = Error;

    fn try_from(val: &XfsMethodResponse) -> Result<Self> {
        val.as_params()?
            .params()
            .iter()
            .map(|m| m.inner())
            .find(|m| m.value().xfs_struct().is_some())
            .ok_or(Error::Xfs(format!(
                "Expected CashUnit XfsMethodResponse, have: {val}"
            )))?
            .value()
            .try_into()
    }
}

impl TryFrom<XfsMethodResponse> for CashUnit {
    type Error = Error;

    fn try_from(val: XfsMethodResponse) -> Result<Self> {
        (&val).try_into()
    }
}
