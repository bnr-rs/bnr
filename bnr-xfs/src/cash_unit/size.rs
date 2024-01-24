use std::fmt;

use crate::impl_xfs_i4;

/// Represents the size of a CU list.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Size(u32);

impl Size {
    /// Creates a new [Size].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Size] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [Size].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [Size].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [Size].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(Size, "size");

/// Represents the maximum size of a CU list.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MaxSize(u32);

impl MaxSize {
    /// Creates a new [MaxSize].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [MaxSize] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        Self(val)
    }

    /// Gets the inner representation of the [MaxSize].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [MaxSize].
    pub fn set_inner(&mut self, val: u32) {
        self.0 = val;
    }

    /// Converts into the inner representation of the [MaxSize].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for MaxSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(MaxSize, "maxSize");
