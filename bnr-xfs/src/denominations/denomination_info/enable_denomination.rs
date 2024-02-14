use std::fmt;

use crate::impl_xfs_bool;

/// Represents whether to enable a denomination specified by a [DenominationInfo](super::DenominationInfo).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnableDenomination(bool);

impl EnableDenomination {
    /// Creates a new [EnableDenomination].
    pub const fn new() -> Self {
        Self(false)
    }

    /// Creates a new [EnableDenomination] from the provided parameter.
    pub const fn create(c: bool) -> Self {
        Self(c)
    }

    /// Gets the inner representation of the [EnableDenomination].
    pub const fn inner(&self) -> bool {
        self.0
    }

    /// Sets the inner representation of the [EnableDenomination].
    pub fn set_inner(&mut self, v: bool) {
        self.0 = v;
    }

    /// Converts into the inner representation of the [EnableDenomination].
    pub fn into_inner(self) -> bool {
        self.0
    }
}

impl Default for EnableDenomination {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EnableDenomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_bool!(EnableDenomination, "enableDenomination");
