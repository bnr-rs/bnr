use std::fmt;

use crate::impl_xfs_enum;

const SEC_COM_LEVEL1: u32 = 0;
const SEC_COM_LEVEL2: u32 = 1;
const SEC_COM_ERROR: u32 = 2;

/// Values for indication of the security level in communication between Host and BNR.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SecuredCommLevel {
    /// Standard communication level.
    #[default]
    Level1 = SEC_COM_LEVEL1,
    /// Secured communication level.
    Level2 = SEC_COM_LEVEL2,
    /// Error occurred.
    Error = SEC_COM_ERROR,
}

impl SecuredCommLevel {
    /// Creates a new [SecuredCommLevel].
    pub const fn new() -> Self {
        Self::Level1
    }

    /// Creates a new [SecuredCommLevel] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            SEC_COM_LEVEL1 => Self::Level1,
            SEC_COM_LEVEL2 => Self::Level2,
            SEC_COM_ERROR => Self::Error,
            _ => Self::Level1,
        }
    }
}

impl From<&SecuredCommLevel> for &'static str {
    fn from(val: &SecuredCommLevel) -> Self {
        match val {
            SecuredCommLevel::Level1 => "level1",
            SecuredCommLevel::Level2 => "level2",
            SecuredCommLevel::Error => "error",
        }
    }
}

impl From<SecuredCommLevel> for &'static str {
    fn from(val: SecuredCommLevel) -> Self {
        (&val).into()
    }
}

impl fmt::Display for SecuredCommLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(SecuredCommLevel, "securedCommLevel");
