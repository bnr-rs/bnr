use std::fmt;

use crate::impl_xfs_struct;
use crate::CashType;

mod enable_denomination;
mod security_level;
mod study_version;

pub use enable_denomination::*;
pub use security_level::*;
pub use study_version::*;

/// Denomination information for enabling and configuring BNR denominations.
#[derive(Clone, Debug, PartialEq)]
pub struct DenominationInfo {
    cash_type: CashType,
    enable_denomination: EnableDenomination,
    security_level: SecurityLevel,
    study_version: StudyVersion,
}

impl DenominationInfo {
    /// Creates a new [DenominationInfo].
    pub const fn new() -> Self {
        Self {
            cash_type: CashType::new(),
            enable_denomination: EnableDenomination::new(),
            security_level: SecurityLevel::new(),
            study_version: StudyVersion::new(),
        }
    }

    /// Creates a new [DenominationInfo] with provided parameters.
    pub const fn create(
        cash_type: CashType,
        enable_denomination: EnableDenomination,
        security_level: SecurityLevel,
        study_version: StudyVersion,
    ) -> Self {
        Self {
            cash_type,
            enable_denomination,
            security_level,
            study_version,
        }
    }

    /// Gets a reference to the [CashType].
    pub const fn cash_type(&self) -> CashType {
        self.cash_type
    }

    /// Sets the [CashType].
    pub fn set_cash_type(&mut self, val: CashType) {
        self.cash_type = val;
    }

    /// Builder function that sets the [CashType].
    pub fn with_cash_type(mut self, val: CashType) -> Self {
        self.set_cash_type(val);
        self
    }

    /// Gets a reference to the [EnableDenomination].
    pub const fn enable_denomination(&self) -> EnableDenomination {
        self.enable_denomination
    }

    /// Sets the [EnableDenomination].
    pub fn set_enable_denomination(&mut self, val: EnableDenomination) {
        self.enable_denomination = val;
    }

    /// Builder function that sets the [EnableDenomination].
    pub fn with_enable_denomination(mut self, val: EnableDenomination) -> Self {
        self.set_enable_denomination(val);
        self
    }

    /// Gets a reference to the [SecurityLevel].
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    /// Sets the [SecurityLevel].
    pub fn set_security_level(&mut self, val: SecurityLevel) {
        self.security_level = val;
    }

    /// Builder function that sets the [SecurityLevel].
    pub fn with_security_level(mut self, val: SecurityLevel) -> Self {
        self.set_security_level(val);
        self
    }

    /// Gets a reference to the [StudyVersion].
    pub const fn study_version(&self) -> &StudyVersion {
        &self.study_version
    }

    /// Sets the [StudyVersion].
    pub fn set_study_version(&mut self, val: StudyVersion) {
        self.study_version = val;
    }

    /// Builder function that sets the [StudyVersion].
    pub fn with_study_version(mut self, val: StudyVersion) -> Self {
        self.set_study_version(val);
        self
    }
}

impl Default for DenominationInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DenominationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""cash_type":{},"#, self.cash_type)?;
        write!(f, r#""enable_denomination":{},"#, self.enable_denomination)?;
        write!(f, r#""security_level":{},"#, self.security_level)?;
        write!(f, r#""study_version":{}"#, self.study_version)?;
        write!(f, "}}")
    }
}

impl_xfs_struct!(
    DenominationInfo,
    "denominationInfo",
    [
        cash_type: CashType,
        enable_denomination: EnableDenomination,
        security_level: SecurityLevel,
        study_version: StudyVersion
    ]
);
