use std::fmt;

use crate::impl_xfs_i4;

/// Represents a CU ID number.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Number(u32);

impl Number {
    /// Creates a new [Number].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Number] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [Number].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [Number].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Number].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(Number, "number");
