use std::fmt;

use crate::impl_xfs_i4;

/// Represents the note count of a CU module.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Count(u32);

impl Count {
    /// Creates a new [Count].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [Count] from the provided parameter.
    pub const fn create(c: u32) -> Self {
        Self(c)
    }

    /// Gets the inner representation of the [Count].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [Count].
    pub fn set_inner(&mut self, v: u32) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [Count].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(Count, "count");

/// Represents the initial note count of a CU module.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InitialCount(u32);

impl InitialCount {
    /// Creates a new [InitialCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [InitialCount] from the provided parameter.
    pub const fn create(c: u32) -> Self {
        Self(c)
    }

    /// Gets the inner representation of the [InitialCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [InitialCount].
    pub fn set_inner(&mut self, v: u32) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [InitialCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for InitialCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(InitialCount, "initialCount");

/// Represents the transport count of a CU module.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransportCount(u32);

impl TransportCount {
    /// Creates a new [TransportCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [TransportCount] from the provided parameter.
    pub const fn create(c: u32) -> Self {
        Self(c)
    }

    /// Gets the inner representation of the [TransportCount].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [TransportCount].
    pub fn set_inner(&mut self, v: u32) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [TransportCount].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for TransportCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(TransportCount, "transportCount");
