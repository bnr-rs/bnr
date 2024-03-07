use std::fmt;

use crate::impl_xfs_struct;
use crate::Count;

use super::*;

mod list;

pub use list::*;

/// Represents a XFS physical cash unit and its parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PhysicalCashUnit {
    name: PcuName,
    unit_id: UnitId,
    count: Count,
    threshold: Threshold,
    status: Status,
    threshold_status: ThresholdStatus,
    threshold_mode: ThresholdMode,
    lock: Lock,
}

impl PhysicalCashUnit {
    /// Creates a new [PhysicalCashUnit].
    pub const fn new() -> Self {
        Self {
            name: PcuName::new(),
            unit_id: UnitId::new(),
            count: Count::new(),
            threshold: Threshold::new(),
            status: Status::new(),
            threshold_status: ThresholdStatus::new(),
            threshold_mode: ThresholdMode::new(),
            lock: Lock::new(),
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
        self.lock.inner()
    }

    /// Sets the lock.
    pub fn set_lock(&mut self, lock: bool) {
        self.lock.set_inner(lock);
    }

    /// Builder function that sets the lock.
    pub fn with_lock(mut self, lock: bool) -> Self {
        self.set_lock(lock);
        self
    }
}

impl fmt::Display for PhysicalCashUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""name": {},"#, self.name)?;
        write!(f, r#""unit_id": {},"#, self.unit_id)?;
        write!(f, r#""count":{},"#, self.count)?;
        write!(f, r#""threshold":{},"#, self.threshold)?;
        write!(f, r#""status":{},"#, self.status)?;
        write!(f, r#""threshold_status":{},"#, self.threshold_status)?;
        write!(f, r#""threshold_mode":{},"#, self.threshold_mode)?;
        write!(f, r#""lock":{}"#, self.lock)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    PhysicalCashUnit,
    "physicalCashUnit",
    [
        name: PcuName,
        unit_id: UnitId,
        count: Count,
        threshold: Threshold,
        status: Status,
        threshold_status: ThresholdStatus,
        threshold_mode: ThresholdMode,
        lock: Lock
    ]
);
