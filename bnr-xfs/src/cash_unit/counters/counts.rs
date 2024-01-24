use std::fmt;

use crate::impl_xfs_i4;

/// Represents the [DepositCounters](super::DepositCounters) deposit count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DepositCount(u32);

impl DepositCount {
    /// Creates a new [DepositCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [DepositCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [DepositCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [DepositCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [DepositCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for DepositCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(DepositCount, "depositCount");

/// Represents the [DepositCounters](super::DepositCounters) retracted count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RetractedCount(u32);

impl RetractedCount {
    /// Creates a new [RetractedCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [RetractedCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [RetractedCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [RetractedCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [RetractedCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for RetractedCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(RetractedCount, "retractedCount");

/// Represents the [DepositCounters](super::DepositCounters) emptied count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EmptiedCount(u32);

impl EmptiedCount {
    /// Creates a new [EmptiedCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [EmptiedCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [EmptiedCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [EmptiedCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [EmptiedCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for EmptiedCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(EmptiedCount, "emptiedCount");

/// Represents the [DepositCounters](super::DepositCounters) forgery count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ForgeryCount(u32);

impl ForgeryCount {
    /// Creates a new [ForgeryCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [ForgeryCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [ForgeryCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [ForgeryCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [ForgeryCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for ForgeryCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(ForgeryCount, "forgeryCount");

/// Represents the [DepositCounters](super::DepositCounters) disappeared count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisappearedCount(u32);

impl DisappearedCount {
    /// Creates a new [DisappearedCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [DisappearedCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [DisappearedCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [DisappearedCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [DisappearedCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for DisappearedCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(DisappearedCount, "disappearedCount");

/// Represents the [DispenseCounters](super::DispenseCounters) reject count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DispenseCount(u32);

impl DispenseCount {
    /// Creates a new [DispenseCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [DispenseCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [DispenseCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [DispenseCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [DispenseCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for DispenseCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(DispenseCount, "dispenseCount");

/// Represents the [DispenseCounters](super::DispenseCounters) reject count.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RejectCount(u32);

impl RejectCount {
    /// Creates a new [RejectCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [RejectCount] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [RejectCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [RejectCount].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [RejectCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for RejectCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(RejectCount, "rejectCount");
