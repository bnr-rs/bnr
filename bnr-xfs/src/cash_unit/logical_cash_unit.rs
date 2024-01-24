use std::fmt;

use crate::currency::{CashType, CashTypeList, CuKind, CuType};
use crate::impl_xfs_struct;

use super::{Count, ExtendedCounters, InitialCount, Number, Status, UnitId};

mod list;

pub use list::*;

/// Represents a logical cash unit, and its parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogicalCashUnit {
    cash_type: CashType,
    secondary_cash_types: CashTypeList,
    number: Number,
    cu_kind: CuKind,
    cu_type: CuType,
    unit_id: UnitId,
    initial_count: InitialCount,
    count: Count,
    status: Status,
    extended_counters: ExtendedCounters,
    physical_cu_index: UnitId,
}

impl LogicalCashUnit {
    /// Creates a new [LogicalCashUnit].
    pub const fn new() -> Self {
        Self {
            cash_type: CashType::new(),
            secondary_cash_types: CashTypeList::new(),
            number: Number::new(),
            cu_kind: CuKind::new(),
            cu_type: CuType::new(),
            unit_id: UnitId::new(),
            initial_count: InitialCount::new(),
            count: Count::new(),
            status: Status::new(),
            extended_counters: ExtendedCounters::new(),
            physical_cu_index: UnitId::new(),
        }
    }

    /// Gets the [CashType].
    ///
    /// Defines the main type of cash used by this cash unit.
    pub const fn cash_type(&self) -> CashType {
        self.cash_type
    }

    /// Sets the [CashType].
    pub fn set_cash_type(&mut self, cash_type: CashType) {
        self.cash_type = cash_type;
    }

    /// Builder function that sets the [CashType].
    pub fn with_cash_type(mut self, cash_type: CashType) -> Self {
        self.set_cash_type(cash_type);
        self
    }

    /// Gets the secondary [CashTypeList].
    ///
    /// Defines the additional types of cash used by this cash unit.
    pub const fn secondary_cash_types(&self) -> &CashTypeList {
        &self.secondary_cash_types
    }

    /// Sets the secondary [CashTypeList].
    pub fn set_secondary_cash_types(&mut self, secondary_cash_types: CashTypeList) {
        self.secondary_cash_types = secondary_cash_types;
    }

    /// Builder function that sets the secondary [CashType].
    pub fn with_secondary_cash_types(mut self, secondary_cash_types: CashTypeList) -> Self {
        self.set_secondary_cash_types(secondary_cash_types);
        self
    }

    /// Gets the logical number of cash unit.
    ///
    /// Unique number of the cash unit. This number is assigned (or reassigned) on bnr_Reset() and identifies the unit along the time; therefore, it can be used to track unit changes, or uniquely reference units in method calls ([DenominationItem::unit](DenominationItem::unit) property is an example).
    pub const fn number(&self) -> u32 {
        self.number.inner()
    }

    /// Sets the number.
    pub fn set_number(&mut self, number: u32) {
        self.number.set_inner(number);
    }

    /// Builder function that sets the number.
    pub fn with_number(mut self, number: u32) -> Self {
        self.set_number(number);
        self
    }

    /// Specifies, if cash unit can dispense, deposit cash or both.
    pub const fn cu_kind(&self) -> CuKind {
        self.cu_kind
    }

    /// Sets the `cu_kind` field, see [LCU].
    pub fn set_cu_kind(&mut self, cu_kind: CuKind) {
        self.cu_kind = cu_kind;
    }

    /// Builder function that sets the cu_kind.
    pub fn with_cu_kind(mut self, cu_kind: CuKind) -> Self {
        self.set_cu_kind(cu_kind);
        self
    }

    /// Gets the type of cash unit.
    pub const fn cu_type(&self) -> CuType {
        self.cu_type
    }

    /// Sets the `cu_type` field, see [LCU].
    pub fn set_cu_type(&mut self, cu_type: CuType) {
        self.cu_type = cu_type;
    }

    /// Builder function that sets the cu_type.
    pub fn with_cu_type(mut self, cu_type: CuType) -> Self {
        self.set_cu_type(cu_type);
        self
    }

    /// Gets the [UnitId].
    pub const fn unit_id(&self) -> UnitId {
        self.unit_id
    }

    /// Sets the [UnitId].
    pub fn set_unit_id(&mut self, unit_id: UnitId) {
        self.unit_id = unit_id;
    }

    /// Builder function that sets the [UnitId].
    pub fn with_unit_id(mut self, unit_id: UnitId) -> Self {
        self.set_unit_id(unit_id);
        self
    }

    /// For customer purpose only; this value is initialized by the [configure_cash_unit](super::configure_cash_unit) and [update_cash_unit](super::update_cash_unit) methods and not changed by the BNR.
    ///
    /// - Type: User data.
    /// - Max: 65535.
    /// - Access: Read-Write (write through [configure_cash_unit](super::configure_cash_unit) and [update_cash_unit](super::update_cash_unit) methods).
    pub const fn initial_count(&self) -> u32 {
        self.initial_count.inner()
    }

    /// Sets the initial count.
    pub fn set_initial_count(&mut self, initial_count: u32) {
        self.initial_count.set_inner(initial_count);
    }

    /// Builder function that sets the initial count.
    pub fn with_initial_count(mut self, initial_count: u32) -> Self {
        self.set_initial_count(initial_count);
        self
    }

    /// Actual count of bills in the logical cash unit.
    ///
    /// - Type: One Shot.
    /// - Max: 65535.
    /// - Access:
    ///   - Bundler and Recycler Physical Cash Units - Read-Only
    ///   - Cashbox and Loader Physical Cash Units - Read-Write (write through [configure_cash_unit](super::configure_cash_unit) and [update_cash_unit](super::update_cash_unit) methods).
    pub const fn count(&self) -> u32 {
        self.count.inner()
    }

    /// Sets the count.
    pub fn set_count(&mut self, count: u32) {
        self.count.set_inner(count);
    }

    /// Builder function that sets the count.
    pub fn with_count(mut self, count: u32) -> Self {
        self.set_count(count);
        self
    }

    /// Cash unit status. Correspond to [PhysicalCashUnit::status].
    pub const fn status(&self) -> u32 {
        self.status.inner()
    }

    /// Sets the status.
    pub fn set_status(&mut self, status: u32) {
        self.status.set_inner(status);
    }

    /// Builder function that sets the status.
    pub fn with_status(mut self, status: u32) -> Self {
        self.set_status(status);
        self
    }

    /// Gets the [ExtendedCounters] for [LCU::Deposit] and [LCU::Dispense] cash unit types.
    pub const fn extended_counters(&self) -> &ExtendedCounters {
        &self.extended_counters
    }

    /// Sets the [ExtendedCounters].
    pub fn set_extended_counters(&mut self, extended_counters: ExtendedCounters) {
        self.extended_counters = extended_counters;
    }

    /// Builder function that sets the extended_counters.
    pub fn with_extended_counters(mut self, extended_counters: ExtendedCounters) -> Self {
        self.set_extended_counters(extended_counters);
        self
    }

    /// Gets the [UnitId] of the [PhysicalCashUnit] backing this [LogicalCashUnit].
    ///
    /// The C library uses a pointer here, but that is wildly unsafe. Use the ID instead.
    pub const fn physical_cu_index(&self) -> UnitId {
        self.physical_cu_index
    }

    /// Sets the physical cash unit [UnitId].
    pub fn set_physical_cu_index(&mut self, pcu_index: UnitId) {
        self.physical_cu_index = pcu_index;
    }

    /// Builder function that sets the physical cash unit [UnitId].
    pub fn with_physical_cash_unit(mut self, pcu_index: UnitId) -> Self {
        self.set_physical_cu_index(pcu_index);
        self
    }
}

impl fmt::Display for LogicalCashUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""cash_type": {}, "#, self.cash_type)?;
        write!(
            f,
            r#""secondary_cash_types": {}, "#,
            self.secondary_cash_types
        )?;
        write!(f, r#""number": {}, "#, self.number)?;
        write!(f, r#""cu_kind": {}, "#, self.cu_kind)?;
        write!(f, r#""cu_type": {}, "#, self.cu_type)?;
        write!(f, r#""unit_id": {}, "#, self.unit_id)?;
        write!(f, r#""initial_count": {}, "#, self.initial_count)?;
        write!(f, r#""count": {}, "#, self.count)?;
        write!(f, r#""status": {}, "#, self.status)?;
        write!(f, r#""extended_counters": {}, "#, self.extended_counters)?;
        write!(f, r#""physical_cu_index": {}"#, self.physical_cu_index)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    LogicalCashUnit,
    "logicalCashUnit",
    [
        cash_type: CashType,
        secondary_cash_types: CashTypeList,
        number: Number,
        cu_kind: CuKind,
        cu_type: CuType,
        unit_id: UnitId,
        initial_count: InitialCount,
        count: Count,
        status: Status,
        extended_counters: ExtendedCounters,
        physical_cu_index: UnitId
    ]
);
