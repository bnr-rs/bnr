use std::fmt;

/// Types of CDR units.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CdrType {
    #[default]
    None = 6010,
    Dispenser = 6011,
    Recycler = 6012,
    Atm = 6013,
}

impl From<u32> for CdrType {
    fn from(val: u32) -> Self {
        match val {
            6010 => Self::None,
            6011 => Self::Dispenser,
            6012 => Self::Recycler,
            6013 => Self::Atm,
            _ => Self::None,
        }
    }
}

impl From<CdrType> for u32 {
    fn from(val: CdrType) -> Self {
        val as u32
    }
}

impl From<&CdrType> for &'static str {
    fn from(val: &CdrType) -> Self {
        match val {
            CdrType::None => "none",
            CdrType::Dispenser => "dispenser",
            CdrType::Recycler => "recycler",
            CdrType::Atm => "atm",
        }
    }
}

impl From<CdrType> for &'static str {
    fn from(val: CdrType) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CdrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
