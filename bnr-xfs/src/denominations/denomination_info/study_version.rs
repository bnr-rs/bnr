use std::fmt;

use crate::impl_xfs_string;

/// Represents the study version of a denomination specified by a [DenominationInfo](super::DenominationInfo).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StudyVersion(String);

impl StudyVersion {
    /// Creates a new [StudyVersion].
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Creates a new [StudyVersion] from the provided parameter.
    pub fn create(c: &str) -> Self {
        Self(c.into())
    }

    /// Gets the inner representation of the [StudyVersion].
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }

    /// Sets the inner representation of the [StudyVersion].
    pub fn set_inner(&mut self, v: &str) {
        self.0 = v.into();
    }

    /// Converts into the inner representation of the [StudyVersion].
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for StudyVersion {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StudyVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl_xfs_string!(StudyVersion, "studyVersion");
