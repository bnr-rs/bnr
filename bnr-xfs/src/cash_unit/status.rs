use std::fmt;

use crate::impl_xfs_i4;

/// Represents the status of a CU module.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Status(u32);

impl Status {
    /// Creates a new [Status].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Status] from the provided parameter.
    pub const fn create(v: u32) -> Self {
        Self(v)
    }

    /// Gets the inner representation of the [Status].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [Status].
    pub fn set_inner(&mut self, v: u32) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [Status].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(Status, "status");
