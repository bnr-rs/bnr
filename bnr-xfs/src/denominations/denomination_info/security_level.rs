use std::fmt;

use crate::impl_xfs_i4;

/// Represents the security level for acceptance of a denomination specified by a [DenominationInfo].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecurityLevel(u32);

impl SecurityLevel {
    /// Creates a new [SecurityLevel].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [SecurityLevel] from the provided parameter.
    pub const fn create(c: u32) -> Self {
        Self(c)
    }

    /// Gets the inner representation of the [SecurityLevel].
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Sets the inner representation of the [SecurityLevel].
    pub fn set_inner(&mut self, v: u32) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [SecurityLevel].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_i4!(SecurityLevel, "securityLevel");
