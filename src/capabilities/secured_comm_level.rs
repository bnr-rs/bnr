use std::fmt;

/// Values for indication of the security level in communication between Host and BNR.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SecuredCommLevel {
    /// Standard communication level.
    #[default]
    Level1 = 0,
    /// Secured communication level.
    Level2,
    /// Error occurred.
    Error,
}

impl SecuredCommLevel {
    /// Creates a new [SecuredCommLevel].
    pub const fn new() -> Self {
        Self::Level1
    }
}

impl From<u32> for SecuredCommLevel {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::Level1,
            1 => Self::Level2,
            2 => Self::Error,
            _ => Self::Level1,
        }
    }
}

impl From<SecuredCommLevel> for u32 {
    fn from(val: SecuredCommLevel) -> Self {
        val as u32
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
