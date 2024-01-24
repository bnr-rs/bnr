use std::fmt;

use crate::{
    arrays,
    currency::{CashType, CashTypeList, LCU},
};

/// Maximum number of [LogicalCashUnit]s that can be present on a device.
pub const LCU_LIST_LEN: usize = 83;

/// Maximum number of [PhysicalCashUnit]s that can be present on a device.
pub const PCU_LIST_LEN: usize = 10;

pub const UNIT_ID_LEN: usize = 20;
pub type UnitId = [i8; UNIT_ID_LEN];

pub const PCU_NAME_LEN: usize = 5;
pub type PcuName = [i8; PCU_NAME_LEN];

/// Convert a C-like char array into a `str`.
macro_rules! c_str {
    ($item:expr) => {{
        std::str::from_utf8($item.map(|i| i as u8).as_ref()).unwrap_or("")
    }};
}

/// Finds a [PhysicalCashUnit] item referenced by the [LogicalCashUnit].
///
/// Returns an optional reference to the [PhysicalCashUnit].
pub fn pcu_for_lcu<'p>(
    lcu_id: UnitId,
    lcu: &LogicalCashUnitList,
    pcu: &'p PhysicalCashUnitList,
) -> Option<&'p PhysicalCashUnit> {
    lcu.items()
        .iter()
        .find(|l| l.unit_id == lcu_id)
        .and_then(|l| {
            pcu.items()
                .iter()
                .find(|p| p.unit_id == l.physical_cash_unit)
        })
}

/// Finds [PhysicalCashUnit] items referenced by the [LogicalCashUnit].
///
/// Returns an optional mutable reference to the [PhysicalCashUnit].
pub fn pcu_for_lcu_mut<'p>(
    lcu_id: UnitId,
    lcu: &LogicalCashUnitList,
    pcu: &'p mut PhysicalCashUnitList,
) -> Option<&'p mut PhysicalCashUnit> {
    lcu.items()
        .iter()
        .find(|l| l.unit_id == lcu_id)
        .and_then(|l| {
            pcu.items_mut()
                .iter_mut()
                .find(|p| p.unit_id == l.physical_cash_unit)
        })
}

pub(crate) fn pcu_for_lcu_sys<'p>(
    lcu_id: u32,
    lcu: &LogicalCashUnitList,
    pcu: &'p mut bnr_sys::PhysicalCashUnitList,
) -> Option<&'p mut bnr_sys::XfsPhysicalCashUnit> {
    let pcu_size = pcu.size as usize;
    lcu.items()
        .iter()
        .find(|l| l.number == lcu_id)
        .and_then(|l| {
            pcu.items[..pcu_size]
                .iter_mut()
                .find(|p| p.unitId == l.physical_cash_unit)
        })
}

/// Sets the [PhysicalCashUnit] pointer in the FFI
/// [LogicalCashUnitList](bnr_sys::LogicalCashUnitList).
///
/// **SAFETY WARNING**: User must ensure not to let the [CashUnit] drop or otherwise invalidate [PhysicalCashUnitList]
/// item references while the `lcu_sys` parameter is in use. For example, do not sort or change the
/// items in the list, do not delete items from the list, etc. This function is safe, but the safety
/// of future use of the `physicalCashUnit` pointer is dependent on the previous constraint.
pub(crate) fn set_pcu_for_lcu(
    lcu_sys: &mut bnr_sys::LogicalCashUnitList,
    lcu_list: &LogicalCashUnitList,
    pcu_list: &mut bnr_sys::PhysicalCashUnitList,
) {
    let lcu_size = lcu_list.size as usize;

    let mut seen: Vec<u32> = Vec::with_capacity(lcu_size);
    let mut dup: Vec<u32> = Vec::with_capacity(lcu_size);

    for lcu in lcu_sys.items[..lcu_sys.size as usize].iter_mut() {
        if let Some(pcu) = pcu_for_lcu_sys(lcu.number, lcu_list, pcu_list) {
            let lcu_num = lcu.number;
            if !seen.contains(&lcu_num) {
                seen.push(lcu_num);
            } else {
                dup.push(lcu_num);
            }

            lcu.physicalCashUnit = pcu as *mut _;
        }
    }

    if !dup.is_empty() {
        log::warn!("Duplicate(s) found: {:x?}", dup);
    }
}

/// Represents a cash unit in a BNR device.
///
/// Describes the entire set of [LogicalCashUnit]s and [PhysicalCashUnit]s present on a device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CashUnit {
    transport_count: u32,
    logical_cash_unit_list: LogicalCashUnitList,
    physical_cash_unit_list: PhysicalCashUnitList,
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

impl CashUnit {
    /// Creates a new [CashUnit].
    pub fn new() -> Self {
        Self {
            transport_count: 0,
            logical_cash_unit_list: LogicalCashUnitList::new(),
            physical_cash_unit_list: PhysicalCashUnitList::new(),
        }
    }

    /// Gets the [PhysicalCashUnit] backing a [LogicalCashUnit].
    ///
    /// Returns an optional reference to the [PhysicalCashUnit].
    pub fn pcu_for_lcu(&self, lcu_id: UnitId) -> Option<&PhysicalCashUnit> {
        pcu_for_lcu(
            lcu_id,
            &self.logical_cash_unit_list,
            &self.physical_cash_unit_list,
        )
    }

    /// Gets the [PhysicalCashUnit] backing a [LogicalCashUnit].
    ///
    /// Returns an optional mutable reference to the [PhysicalCashUnit].
    pub fn pcu_for_lcu_mut(&mut self, lcu_id: UnitId) -> Option<&mut PhysicalCashUnit> {
        pcu_for_lcu_mut(
            lcu_id,
            &self.logical_cash_unit_list,
            &mut self.physical_cash_unit_list,
        )
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

impl From<&CashUnit> for bnr_sys::XfsCashUnit {
    fn from(val: &CashUnit) -> Self {
        let mut logical_cash_unit_list =
            bnr_sys::LogicalCashUnitList::from(val.logical_cash_unit_list);

        let mut physical_cash_unit_list =
            bnr_sys::PhysicalCashUnitList::from(val.physical_cash_unit_list);

        set_pcu_for_lcu(
            &mut logical_cash_unit_list,
            val.logical_cash_unit_list(),
            &mut physical_cash_unit_list,
        );

        Self {
            transportCount: val.transport_count,
            logicalCashUnitList: logical_cash_unit_list,
            physicalCashUnitList: physical_cash_unit_list,
        }
    }
}

impl From<CashUnit> for bnr_sys::XfsCashUnit {
    fn from(val: CashUnit) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsCashUnit> for CashUnit {
    fn from(val: &bnr_sys::XfsCashUnit) -> Self {
        Self {
            transport_count: val.transportCount,
            logical_cash_unit_list: val.logicalCashUnitList.into(),
            physical_cash_unit_list: val.physicalCashUnitList.into(),
        }
    }
}

impl From<bnr_sys::XfsCashUnit> for CashUnit {
    fn from(val: bnr_sys::XfsCashUnit) -> Self {
        (&val).into()
    }
}

/// Represents a list of [LogicalCashUnit]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogicalCashUnitList {
    max_size: u32,
    size: u32,
    #[serde(with = "arrays")]
    items: [LogicalCashUnit; LCU_LIST_LEN],
}

impl LogicalCashUnitList {
    /// Creates a new [LogicalCashUnitList].
    pub const fn new() -> Self {
        Self {
            max_size: LCU_LIST_LEN as u32,
            size: 0,
            items: [LogicalCashUnit::new(); LCU_LIST_LEN],
        }
    }

    /// Gets the size of the items list.
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Sets the size of the items list.
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Builder function that sets the size of the items list.
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets a list of the [LogicalCashUnit]s.
    pub fn items(&self) -> &[LogicalCashUnit] {
        let size = self.size as usize;
        if size <= self.items.len() {
            self.items[..size].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable list of the [LogicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [LogicalCashUnit] {
        let size = self.size as usize;
        if size <= self.items.len() {
            self.items[..size].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets a list of [LogicalCashUnit]s.
    pub fn set_items(&mut self, items: [LogicalCashUnit; LCU_LIST_LEN]) {
        self.items = items;
    }

    /// Builder function that sets a list of [LogicalCashUnit]s.
    pub fn with_items(mut self, items: [LogicalCashUnit; LCU_LIST_LEN]) -> Self {
        self.set_items(items);
        self
    }
}

impl Default for LogicalCashUnitList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&LogicalCashUnitList> for bnr_sys::LogicalCashUnitList {
    fn from(val: &LogicalCashUnitList) -> Self {
        Self {
            maxSize: val.max_size,
            size: val.size,
            items: val.items.map(bnr_sys::XfsLogicalCashUnit::from),
        }
    }
}

impl From<LogicalCashUnitList> for bnr_sys::LogicalCashUnitList {
    fn from(val: LogicalCashUnitList) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::LogicalCashUnitList> for LogicalCashUnitList {
    fn from(val: &bnr_sys::LogicalCashUnitList) -> Self {
        Self {
            max_size: val.maxSize,
            size: val.size,
            items: val.items.map(LogicalCashUnit::from),
        }
    }
}

impl From<bnr_sys::LogicalCashUnitList> for LogicalCashUnitList {
    fn from(val: bnr_sys::LogicalCashUnitList) -> Self {
        (&val).into()
    }
}

impl fmt::Display for LogicalCashUnitList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size": {},"#, self.max_size)?;
        write!(f, r#""size": {},"#, self.size)?;

        write!(f, r#""items": ["#)?;
        let items_len = self.items().len();
        for (i, item) in self.items().iter().enumerate() {
            write!(f, "{item}")?;

            if i != items_len - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]}}")
    }
}

/// Represents a logical cash unit, and its parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogicalCashUnit {
    cash_type: CashType,
    secondary_cash_types: CashTypeList,
    number: u32,
    cu_kind: LCU,
    cu_type: LCU,
    unit_id: UnitId,
    initial_count: u32,
    count: u32,
    status: u32,
    extended_counters: ExtendedCounters,
    physical_cash_unit: UnitId,
}

impl LogicalCashUnit {
    /// Creates a new [LogicalCashUnit].
    pub const fn new() -> Self {
        Self {
            cash_type: CashType::new(),
            secondary_cash_types: CashTypeList::new(),
            number: 0,
            cu_kind: LCU::NA,
            cu_type: LCU::NA,
            unit_id: [0; UNIT_ID_LEN],
            initial_count: 0,
            count: 0,
            status: 0,
            extended_counters: ExtendedCounters::new(),
            physical_cash_unit: [0; UNIT_ID_LEN],
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
        self.number
    }

    /// Sets the number.
    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }

    /// Builder function that sets the number.
    pub fn with_number(mut self, number: u32) -> Self {
        self.set_number(number);
        self
    }

    /// Specifies, if cash unit can dispense, deposit cash or both.
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::Deposit]
    /// - [LCU::Dispense]
    /// - [LCU::Recycle]
    pub const fn cu_kind(&self) -> LCU {
        self.cu_kind
    }

    /// Sets the `cu_kind` field, see [LCU].
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::Deposit]
    /// - [LCU::Dispense]
    /// - [LCU::Recycle]
    ///
    /// No-op if the `cu_kind` parameter is invalid.
    pub fn set_cu_kind(&mut self, cu_kind: LCU) {
        if matches!(
            cu_kind,
            LCU::NA | LCU::Deposit | LCU::Dispense | LCU::Recycle
        ) {
            self.cu_kind = cu_kind;
        }
    }

    /// Builder function that sets the cu_kind.
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::Deposit]
    /// - [LCU::Dispense]
    /// - [LCU::Recycle]
    ///
    /// No-op if the `cu_kind` parameter is invalid.
    pub fn with_cu_kind(mut self, cu_kind: LCU) -> Self {
        self.set_cu_kind(cu_kind);
        self
    }

    /// Gets the type of cash unit.
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::BillCassette]
    /// - [LCU::RejectCassette]
    pub const fn cu_type(&self) -> LCU {
        self.cu_type
    }

    /// Sets the `cu_type` field, see [LCU].
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::BillCassette]
    /// - [LCU::RejectCassette]
    ///
    /// No-op if the `cu_type` parameter is invalid.
    pub fn set_cu_type(&mut self, cu_type: LCU) {
        if matches!(cu_type, LCU::NA | LCU::BillCassette | LCU::RejectCassette) {
            self.cu_type = cu_type;
        }
    }

    /// Builder function that sets the cu_type.
    ///
    /// One of the following values:
    /// - [LCU::NA]
    /// - [LCU::BillCassette]
    /// - [LCU::RejectCassette]
    ///
    /// No-op if the `cu_type` parameter is invalid.
    pub fn with_cu_type(mut self, cu_type: LCU) -> Self {
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
        self.initial_count
    }

    /// Sets the initial count.
    pub fn set_initial_count(&mut self, initial_count: u32) {
        self.initial_count = initial_count;
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
        self.count
    }

    /// Sets the count.
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    /// Builder function that sets the count.
    pub fn with_count(mut self, count: u32) -> Self {
        self.set_count(count);
        self
    }

    /// Cash unit status. Correspond to [PhysicalCashUnit::status].
    pub const fn status(&self) -> u32 {
        self.status
    }

    /// Sets the status.
    pub fn set_status(&mut self, status: u32) {
        self.status = status;
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
    pub const fn physical_cash_unit(&self) -> UnitId {
        self.physical_cash_unit
    }

    /// Sets the physical cash unit [UnitId].
    pub fn set_physical_cash_unit(&mut self, physical_cash_unit: UnitId) {
        self.physical_cash_unit = physical_cash_unit;
    }

    /// Builder function that sets the physical cash unit [UnitId].
    pub fn with_physical_cash_unit(mut self, physical_cash_unit: UnitId) -> Self {
        self.set_physical_cash_unit(physical_cash_unit);
        self
    }
}

impl From<&LogicalCashUnit> for bnr_sys::XfsLogicalCashUnit {
    fn from(val: &LogicalCashUnit) -> Self {
        Self {
            cashType: val.cash_type.into(),
            secondaryCashTypes: val.secondary_cash_types.into(),
            number: val.number,
            cuKind: val.cu_kind.into(),
            cuType: val.cu_type.into(),
            unitId: val.unit_id,
            initialCount: val.initial_count,
            count: val.count,
            status: val.status,
            extendedCounters: val.extended_counters.into(),
            // Leave the reference null, and perform the actual mapping in the CashUnit conversion
            physicalCashUnit: std::ptr::null_mut(),
        }
    }
}

impl From<LogicalCashUnit> for bnr_sys::XfsLogicalCashUnit {
    fn from(val: LogicalCashUnit) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsLogicalCashUnit> for LogicalCashUnit {
    fn from(val: &bnr_sys::XfsLogicalCashUnit) -> Self {
        let cu_type = LCU::from(val.cuType);
        // SAFETY: the union reads below are safe because all fields in the union have a `u32`
        // type, and the only difference between the variants are the number of `u32` fields.
        //
        // Since unions allocate for the widest size, both accesses are safe. For example, if the
        // C library returns a `Deposit` for `cuType`, but the union actually holds a
        // `DispenseCounters` struct, reading the extra fields will only result in meaningless
        // values in the resulting `DepositCounters` fields, not undefined behavior.
        let extended_counters = match cu_type {
            LCU::Deposit => ExtendedCounters::from(unsafe { val.extendedCounters.deposit }),
            LCU::Dispense => ExtendedCounters::from(unsafe { val.extendedCounters.dispense }),
            _ => ExtendedCounters::new(),
        };

        Self {
            cash_type: val.cashType.into(),
            secondary_cash_types: val.secondaryCashTypes.into(),
            number: val.number,
            cu_kind: val.cuKind.into(),
            cu_type,
            unit_id: val.unitId,
            initial_count: val.initialCount,
            count: val.count,
            status: val.status,
            extended_counters,
            physical_cash_unit: if !val.physicalCashUnit.is_null() {
                unsafe { (*val.physicalCashUnit).unitId }
            } else {
                [0; UNIT_ID_LEN]
            },
        }
    }
}

impl From<bnr_sys::XfsLogicalCashUnit> for LogicalCashUnit {
    fn from(val: bnr_sys::XfsLogicalCashUnit) -> Self {
        (&val).into()
    }
}

impl fmt::Display for LogicalCashUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""cash_type":{},"#, self.cash_type)?;
        write!(
            f,
            r#""secondary_cash_types":{},"#,
            self.secondary_cash_types
        )?;
        write!(f, r#""number":{},"#, self.number)?;
        write!(f, r#""cu_kind":{},"#, self.cu_kind)?;
        write!(f, r#""cu_type":{},"#, self.cu_type)?;
        write!(f, r#""unit_id":{},"#, c_str!(&self.unit_id))?;
        write!(f, r#""initial_count":{},"#, self.initial_count)?;
        write!(f, r#""count":{},"#, self.count)?;
        write!(f, r#""status":{},"#, self.status)?;
        write!(f, r#""extended_counters":{},"#, self.extended_counters)?;
        write!(
            f,
            r#""physical_cash_unit":{}"#,
            c_str!(&self.physical_cash_unit)
        )?;
        write!(f, "}}")
    }
}

/// Represents extended counters for a cash unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ExtendedCounters {
    deposit: Option<DepositCounters>,
    dispense: Option<DispenseCounters>,
}

impl ExtendedCounters {
    /// Creates a new [ExtendedCounters].
    pub const fn new() -> Self {
        Self {
            deposit: None,
            dispense: None,
        }
    }
}

impl From<&ExtendedCounters> for bnr_sys::ExtendedCounters {
    fn from(val: &ExtendedCounters) -> Self {
        match val {
            ExtendedCounters {
                deposit: Some(d),
                dispense: _,
            } => Self { deposit: d.into() },
            ExtendedCounters {
                deposit: None,
                dispense: Some(d),
            } => Self { dispense: d.into() },
            _ => Self {
                deposit: DepositCounters::new().into(),
            },
        }
    }
}

impl From<ExtendedCounters> for bnr_sys::ExtendedCounters {
    fn from(val: ExtendedCounters) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::DepositCounters> for ExtendedCounters {
    fn from(val: &bnr_sys::DepositCounters) -> Self {
        Self {
            deposit: Some(val.into()),
            dispense: None,
        }
    }
}

impl From<bnr_sys::DepositCounters> for ExtendedCounters {
    fn from(val: bnr_sys::DepositCounters) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::DispenseCounters> for ExtendedCounters {
    fn from(val: &bnr_sys::DispenseCounters) -> Self {
        Self {
            deposit: None,
            dispense: Some(val.into()),
        }
    }
}

impl From<bnr_sys::DispenseCounters> for ExtendedCounters {
    fn from(val: bnr_sys::DispenseCounters) -> Self {
        (&val).into()
    }
}

impl From<DepositCounters> for ExtendedCounters {
    fn from(val: DepositCounters) -> Self {
        Self {
            deposit: Some(val),
            dispense: None,
        }
    }
}

impl From<&DepositCounters> for ExtendedCounters {
    fn from(val: &DepositCounters) -> Self {
        (*val).into()
    }
}

impl From<DispenseCounters> for ExtendedCounters {
    fn from(val: DispenseCounters) -> Self {
        Self {
            deposit: None,
            dispense: Some(val),
        }
    }
}

impl From<&DispenseCounters> for ExtendedCounters {
    fn from(val: &DispenseCounters) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ExtendedCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        if let Some(d) = self.deposit.as_ref() {
            write!(f, r#""deposit":{d}"#)?;
        }

        if let Some(d) = self.dispense.as_ref() {
            if self.deposit.is_some() {
                write!(f, ",")?;
            }

            write!(f, r#""dispense":{d}"#)?;
        }

        write!(f, "}}")
    }
}

/// Represents counters for deposits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DepositCounters {
    deposit_count: u32,
    retracted_count: u32,
    emptied_count: u32,
    forgery_count: u32,
    disappeared_count: u32,
}

impl DepositCounters {
    /// Creates a new [DepositCounters].
    pub const fn new() -> Self {
        Self {
            deposit_count: 0,
            retracted_count: 0,
            emptied_count: 0,
            forgery_count: 0,
            disappeared_count: 0,
        }
    }
}

impl From<&DepositCounters> for bnr_sys::DepositCounters {
    fn from(val: &DepositCounters) -> Self {
        Self {
            depositCount: val.deposit_count,
            retractedCount: val.retracted_count,
            emptiedCount: val.emptied_count,
            forgeryCount: val.forgery_count,
            disappearedCount: val.disappeared_count,
        }
    }
}

impl From<DepositCounters> for bnr_sys::DepositCounters {
    fn from(val: DepositCounters) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::DepositCounters> for DepositCounters {
    fn from(val: &bnr_sys::DepositCounters) -> Self {
        Self {
            deposit_count: val.depositCount,
            retracted_count: val.retractedCount,
            emptied_count: val.emptiedCount,
            forgery_count: val.forgeryCount,
            disappeared_count: val.disappearedCount,
        }
    }
}

impl From<bnr_sys::DepositCounters> for DepositCounters {
    fn from(val: bnr_sys::DepositCounters) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DepositCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""deposit_count":{}"#, self.deposit_count)?;
        write!(f, r#""retracted_count":{}"#, self.retracted_count)?;
        write!(f, r#""emptied_count":{}"#, self.emptied_count)?;
        write!(f, r#""forgery_count":{}"#, self.forgery_count)?;
        write!(f, r#""disappeared_count":{}"#, self.disappeared_count)?;
        write!(f, "}}")
    }
}

/// Represents counters for dispensed notes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DispenseCounters {
    dispense_count: u32,
    reject_count: u32,
}

impl DispenseCounters {
    /// Creates a new [DispenseCounters].
    pub const fn new() -> Self {
        Self {
            dispense_count: 0,
            reject_count: 0,
        }
    }
}

impl From<&DispenseCounters> for bnr_sys::DispenseCounters {
    fn from(val: &DispenseCounters) -> Self {
        Self {
            dispenseCount: val.dispense_count,
            rejectCount: val.reject_count,
        }
    }
}

impl From<DispenseCounters> for bnr_sys::DispenseCounters {
    fn from(val: DispenseCounters) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::DispenseCounters> for DispenseCounters {
    fn from(val: &bnr_sys::DispenseCounters) -> Self {
        Self {
            dispense_count: val.dispenseCount,
            reject_count: val.rejectCount,
        }
    }
}

impl From<bnr_sys::DispenseCounters> for DispenseCounters {
    fn from(val: bnr_sys::DispenseCounters) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DispenseCounters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""dispense_count":{}"#, self.dispense_count)?;
        write!(f, r#""reject_count":{}"#, self.reject_count)?;
        write!(f, "}}")
    }
}

/// Represents a list of [PhysicalCashUnit]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PhysicalCashUnitList {
    max_size: u32,
    size: u32,
    items: [PhysicalCashUnit; PCU_LIST_LEN],
}

impl PhysicalCashUnitList {
    /// Creates a new [PhysicalCashUnitList].
    pub const fn new() -> Self {
        Self {
            max_size: PCU_LIST_LEN as u32,
            size: 0,
            items: [PhysicalCashUnit::new(); PCU_LIST_LEN],
        }
    }

    /// Gets the size.
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Sets the size.
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Builder function that sets the size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets a list of the [PhysicalCashUnit]s.
    pub fn items(&self) -> &[PhysicalCashUnit] {
        let size = self.size as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..self.size as usize].as_ref()
        } else {
            self.items.as_ref()
        }
    }

    /// Gets a mutable list of the [PhysicalCashUnit]s.
    pub fn items_mut(&mut self) -> &mut [PhysicalCashUnit] {
        let size = self.size as usize;

        if (0..self.items.len()).contains(&size) {
            self.items[..self.size as usize].as_mut()
        } else {
            self.items.as_mut()
        }
    }

    /// Sets a list of the [PhysicalCashUnit]s.
    pub fn set_items(&mut self, items: [PhysicalCashUnit; PCU_LIST_LEN]) {
        self.items = items;
    }

    /// Builder function that sets a list of the [PhysicalCashUnit]s.
    pub fn with_items(mut self, items: [PhysicalCashUnit; PCU_LIST_LEN]) -> Self {
        self.set_items(items);
        self
    }
}

impl From<&bnr_sys::PhysicalCashUnitList> for PhysicalCashUnitList {
    fn from(val: &bnr_sys::PhysicalCashUnitList) -> Self {
        Self {
            max_size: val.maxSize,
            size: val.size,
            items: val.items.map(PhysicalCashUnit::from),
        }
    }
}

impl From<bnr_sys::PhysicalCashUnitList> for PhysicalCashUnitList {
    fn from(val: bnr_sys::PhysicalCashUnitList) -> Self {
        (&val).into()
    }
}

impl From<&PhysicalCashUnitList> for bnr_sys::PhysicalCashUnitList {
    fn from(val: &PhysicalCashUnitList) -> Self {
        Self {
            maxSize: val.max_size,
            size: val.size,
            items: val.items.map(bnr_sys::XfsPhysicalCashUnit::from),
        }
    }
}

impl From<PhysicalCashUnitList> for bnr_sys::PhysicalCashUnitList {
    fn from(val: PhysicalCashUnitList) -> Self {
        (&val).into()
    }
}

impl fmt::Display for PhysicalCashUnitList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""max_size":{}"#, self.max_size)?;
        write!(f, r#""size":{}"#, self.size)?;

        write!(f, r#""items":["#)?;
        let item_len = self.items().len();
        for (i, item) in self.items().iter().enumerate() {
            write!(f, r#"{item}"#)?;

            if i != item_len - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]}}")
    }
}

/// Represents a XFS physical cash unit and its parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PhysicalCashUnit {
    name: PcuName,
    unit_id: UnitId,
    count: u32,
    threshold: Threshold,
    status: u32,
    threshold_status: ThresholdStatus,
    threshold_mode: ThresholdMode,
    lock: bool,
}

impl PhysicalCashUnit {
    /// Creates a new [PhysicalCashUnit].
    pub const fn new() -> Self {
        Self {
            name: [0; PCU_NAME_LEN],
            unit_id: [0; UNIT_ID_LEN],
            count: 0,
            threshold: Threshold::new(),
            status: 0,
            threshold_status: ThresholdStatus::new(),
            threshold_mode: ThresholdMode::new(),
            lock: false,
        }
    }
    /// Gets the [PcuName].
    ///
    /// Name of the physical location in the BNR where this cash unit is installed.
    pub const fn name(&self) -> &PcuName {
        &self.name
    }

    /// Sets the [PcuName].
    pub fn set_name(&mut self, name: PcuName) {
        self.name = name;
    }

    /// Builder function that sets the [PcuName].
    pub fn with_name(mut self, name: PcuName) -> Self {
        self.set_name(name);
        self
    }

    /// Gets the [UnitId].
    ///
    /// Physical cash unit ID. Corresponds to the BNR Module Serial Number (MSN).
    pub const fn unit_id(&self) -> &UnitId {
        &self.unit_id
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

    /// Gets the count.
    ///
    /// Actual count of bills in the physical cash unit.
    /// - Type: One Shot.
    /// - Max: 65535.
    /// - Access:
    ///   - Bundler and Recycler Physical Cash Units - Read-Only
    ///   - Cashbox and Loader Physical Cash Units - Read-Write (write through [configure_cash_unit](super::configure_cash_unit) and [update_cash_unit](super::update_cash_unit) methods).
    pub const fn count(&self) -> u32 {
        self.count
    }

    /// Sets the count.
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    /// Builder function that sets the count.
    pub fn with_count(mut self, count: u32) -> Self {
        self.set_count(count);
        self
    }

    /// Gets the [Threshold].
    ///
    /// Defines limits to determine [threshold_status](Self::threshold_status).
    pub const fn threshold(&self) -> Threshold {
        self.threshold
    }

    /// Sets the [Threshold].
    pub fn set_threshold(&mut self, threshold: Threshold) {
        self.threshold = threshold;
    }

    /// Builder function that sets the [Threshold].
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.set_threshold(threshold);
        self
    }

    /// Gets the status.
    ///
    /// Status of the physical cash unit.
    pub const fn status(&self) -> u32 {
        self.status
    }

    /// Sets the status.
    pub fn set_status(&mut self, status: u32) {
        self.status = status;
    }

    /// Builder function that sets the status.
    pub fn with_status(mut self, status: u32) -> Self {
        self.set_status(status);
        self
    }

    /// Gets the [ThresholdStatus].
    ///
    /// [ThresholdStatus] of the physical cash unit.
    pub const fn threshold_status(&self) -> ThresholdStatus {
        self.threshold_status
    }

    /// Sets the [ThresholdStatus].
    pub fn set_threshold_status(&mut self, status: ThresholdStatus) {
        self.threshold_status = status;
    }

    /// Builder function that sets the [ThresholdStatus].
    pub fn with_threshold_status(mut self, status: ThresholdStatus) -> Self {
        self.set_threshold_status(status);
        self
    }

    /// Gets the [ThresholdMode].
    ///
    /// Defines how [threshold_status](Self::threshold_status) is determined.
    pub const fn threshold_mode(&self) -> ThresholdMode {
        self.threshold_mode
    }

    /// Sets the [ThresholdMode].
    pub fn set_threshold_mode(&mut self, mode: ThresholdMode) {
        self.threshold_mode = mode;
    }

    /// Builder function that sets the [ThresholdMode].
    pub fn with_threshold_mode(mut self, mode: ThresholdMode) -> Self {
        self.set_threshold_mode(mode);
        self
    }

    /// Gets the lock.
    ///
    /// Enables or disables the physical cash unit.
    pub const fn lock(&self) -> bool {
        self.lock
    }

    /// Sets the lock.
    pub fn set_lock(&mut self, lock: bool) {
        self.lock = lock;
    }

    /// Builder function that sets the lock.
    pub fn with_lock(mut self, lock: bool) -> Self {
        self.set_lock(lock);
        self
    }
}

impl From<&bnr_sys::XfsPhysicalCashUnit> for PhysicalCashUnit {
    fn from(val: &bnr_sys::XfsPhysicalCashUnit) -> Self {
        Self {
            name: val.name,
            unit_id: val.unitId,
            count: val.count,
            threshold: val.threshold.into(),
            status: val.status,
            threshold_status: val.thresholdStatus.into(),
            threshold_mode: val.thresholdMode.into(),
            lock: val.lock != 0,
        }
    }
}

impl From<bnr_sys::XfsPhysicalCashUnit> for PhysicalCashUnit {
    fn from(val: bnr_sys::XfsPhysicalCashUnit) -> Self {
        (&val).into()
    }
}

impl From<&PhysicalCashUnit> for bnr_sys::XfsPhysicalCashUnit {
    fn from(val: &PhysicalCashUnit) -> Self {
        Self {
            name: val.name,
            unitId: val.unit_id,
            count: val.count,
            threshold: val.threshold.into(),
            status: val.status,
            thresholdStatus: val.threshold_status.into(),
            thresholdMode: val.threshold_mode.into(),
            lock: val.lock.into(),
        }
    }
}

impl From<PhysicalCashUnit> for bnr_sys::XfsPhysicalCashUnit {
    fn from(val: PhysicalCashUnit) -> Self {
        (&val).into()
    }
}

impl fmt::Display for PhysicalCashUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""name":"{}","#, c_str!(&self.name))?;
        write!(f, r#""unit_id":"{}","#, c_str!(&self.unit_id))?;
        write!(f, r#""count":{},"#, self.count)?;
        write!(f, r#""threshold":{},"#, self.threshold)?;
        write!(f, r#""status":{},"#, self.status)?;
        write!(f, r#""threshold_status":{},"#, self.threshold_status)?;
        write!(f, r#""threshold_mode":{},"#, self.threshold_mode)?;
        write!(f, r#""lock":{}"#, self.lock)?;
        write!(f, "}}")
    }
}

/// Structure that defines the levels determining a physical cash unit [ThresholdStatus].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Threshold {
    /// The PCU [ThresholdStatus] becomes Full when the bill count is greater or equal to this value.
    full: u32,
    /// The PCU [ThresholdStatus] becomes High when the bill count is greater than this value.
    high: u32,
    /// The PCU [ThresholdStatus] becomes Low when the bill count is lower to this value.
    low: u32,
    /// The PCU [ThresholdStatus] becomes Empty when the bill count is lower or equal to this value.
    empty: u32,
}

impl Threshold {
    /// Creates a new [Threshold].
    pub const fn new() -> Self {
        Self {
            full: 0,
            high: 0,
            low: 0,
            empty: 0,
        }
    }

    /// Gets the full threshold limit.
    pub const fn full(&self) -> u32 {
        self.full
    }

    /// Gets the high threshold limit.
    pub const fn high(&self) -> u32 {
        self.high
    }

    /// Gets the low threshold limit.
    pub const fn low(&self) -> u32 {
        self.low
    }

    /// Gets the empty threshold limit.
    pub const fn empty(&self) -> u32 {
        self.empty
    }
}

impl From<&bnr_sys::XfsThreshold> for Threshold {
    fn from(val: &bnr_sys::XfsThreshold) -> Self {
        Self {
            full: val.full,
            high: val.high,
            low: val.low,
            empty: val.empty,
        }
    }
}

impl From<bnr_sys::XfsThreshold> for Threshold {
    fn from(val: bnr_sys::XfsThreshold) -> Self {
        (&val).into()
    }
}

impl From<&Threshold> for bnr_sys::XfsThreshold {
    fn from(val: &Threshold) -> Self {
        Self {
            full: val.full(),
            high: val.high(),
            low: val.low(),
            empty: val.empty(),
        }
    }
}

impl From<Threshold> for bnr_sys::XfsThreshold {
    fn from(val: Threshold) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Threshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""full":{},"#, self.full)?;
        write!(f, r#""high":{},"#, self.high)?;
        write!(f, r#""low":{},"#, self.low)?;
        write!(f, r#""empty":{}"#, self.empty)?;
        write!(f, "}}")
    }
}

/// Filling status of a cash unit.
///
/// How the threshold status of a cash unit is determined, depends of the threshold mode :
///
/// @par SensorMode (default value):
/// ThresholdStatus changes are determined by the sensors of the BNR (physical filling status).
///
/// - `CountMode`: The management of these values depends of the physical cash unit type
///  - Cashbox, Recycler and Bundler: Based on the Threshold levels, but if the physical limit of filling is reached before the threshold Full, then the ThresholdStatus is forced to Full anyway.
///  - Loader: Based on the Threshold levels, but if there is a lack of bills before the threshold Empty, the ThresholdStatus is forced to Empty anyway.
///
/// **see** [Threshold], [ThresholdMode], [PhysicalCashUnit].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ThresholdStatus {
    /// The cash unit is neither empty nor full.
    #[default]
    Ok = 0,
    /// The cash unit is full. In CountMode, the cash unit count is higher or equal to its full threshold level.
    Full = 1,
    /// The cash unit is alomst full. In CountMode, the cash unit count is higher than its high threshold level.
    High = 2,
    /// The cash unit is almost empty. In CountMode, the cash unit count is lower than its low threshold level.
    Low = 4,
    /// The cash unit is empty. In CountMode, the cash unit count is lower or equal to its empty threshold level.
    Empty = 8,
    /// Threshold state cannot be determined.
    Unknown = 16,
    /// Threshold state is not supported.
    NotSupported = 32,
}

impl ThresholdStatus {
    /// Creates a new [ThresholdStatus].
    pub const fn new() -> Self {
        Self::Ok
    }
}

impl From<u32> for ThresholdStatus {
    fn from(val: u32) -> Self {
        match val {
            v if v == 0 => Self::Ok,
            v if v == 1 => Self::Full,
            v if v == 2 => Self::High,
            v if v == 4 => Self::Low,
            v if v == 8 => Self::Empty,
            v if v == 16 => Self::Unknown,
            v if v == 32 => Self::NotSupported,
            _ => Self::Unknown,
        }
    }
}

impl From<ThresholdStatus> for u32 {
    fn from(val: ThresholdStatus) -> Self {
        val as u32
    }
}

impl From<&ThresholdStatus> for &'static str {
    fn from(val: &ThresholdStatus) -> Self {
        match val {
            ThresholdStatus::Ok => "OK",
            ThresholdStatus::Full => "full",
            ThresholdStatus::High => "high",
            ThresholdStatus::Low => "low",
            ThresholdStatus::Empty => "empty",
            ThresholdStatus::Unknown => "unknown",
            ThresholdStatus::NotSupported => "not supported",
        }
    }
}

impl From<ThresholdStatus> for &'static str {
    fn from(val: ThresholdStatus) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ThresholdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Threshold mode used to determine the [ThresholdStatus] of a PCU.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ThresholdMode {
    /// [ThresholdStatus] changes are determined by the sensors of the BNR (physical filling status).
    #[default]
    Sensor = 0,
    /// [ThresholdStatus] changes are determined by comparing the PCU counts to the Threshold levels.
    Count,
}

impl ThresholdMode {
    /// Creates a new [ThresholdMode].
    pub const fn new() -> Self {
        Self::Sensor
    }
}

impl From<u32> for ThresholdMode {
    fn from(val: u32) -> Self {
        match val {
            v if v == 0 => Self::Sensor,
            v if v == 1 => Self::Count,
            _ => Self::Sensor,
        }
    }
}

impl From<ThresholdMode> for u32 {
    fn from(val: ThresholdMode) -> Self {
        val as u32
    }
}

impl From<&ThresholdMode> for &'static str {
    fn from(val: &ThresholdMode) -> Self {
        match val {
            ThresholdMode::Sensor => "sensor",
            ThresholdMode::Count => "count",
        }
    }
}

impl From<ThresholdMode> for &'static str {
    fn from(val: ThresholdMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ThresholdMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
