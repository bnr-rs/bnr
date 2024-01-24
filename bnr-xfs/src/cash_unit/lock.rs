use std::fmt;

use crate::impl_xfs_bool;

/// Representation of the `lock` field of a [PhysicalCashUnit](super::PhysicalCashUnit).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Lock(bool);

impl Lock {
    /// Creates a new [Lock].
    pub const fn new() -> Self {
        Self(false)
    }

    /// Creates a new [Lock] from the provided parameter.
    pub const fn create(v: bool) -> Self {
        Self(v)
    }

    /// Gets the inner representation of the [Lock].
    pub const fn inner(&self) -> bool {
        self.0
    }

    /// Gets the inner representation of the [Lock].
    pub fn set_inner(&mut self, v: bool) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [Lock].
    pub fn into_inner(self) -> bool {
        self.0
    }
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_bool!(Lock, "lock");
