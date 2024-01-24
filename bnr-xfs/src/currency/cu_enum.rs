use std::fmt;

use crate::impl_xfs_enum;

pub const LCU_DISPENSE: u32 = 6016;
pub const LCU_DEPOSIT: u32 = 6017;
pub const LCU_RECYCLE: u32 = 6018;
pub const LCU_NA: u32 = 6019;
pub const LCU_BAIT_TRAP: u32 = 6020;
pub const LCU_REJECT_CASSETTE: u32 = 6021;
pub const LCU_OVERFLOW_CASSETTE: u32 = 6022;
pub const LCU_BILL_CASSETTE: u32 = 6023;
pub const LCU_COIN_CYLINDER: u32 = 6024;
pub const LCU_COIN_DISPENSER: u32 = 6025;
pub const LCU_RETRACT_CASSETTE: u32 = 6026;
pub const LCU_COUPON: u32 = 6027;
pub const LCU_DOCUMENT: u32 = 6028;
pub const LCU_ESCROW: u32 = 6029;
pub const LCU_UNKNOWN: u32 = 6030;
pub const LCU_OK: u32 = 6031;
pub const LCU_FULL: u32 = 6032;
pub const LCU_HIGH: u32 = 6033;
pub const LCU_LOW: u32 = 6034;
pub const LCU_EMPTY: u32 = 6035;
pub const LCU_INOP: u32 = 6036;
pub const LCU_MISSING: u32 = 6037;
pub const LCU_NO_VALUE: u32 = 6038;
pub const LCU_NO_REF: u32 = 6039;
pub const LCU_NOT_DISPENSABLE: u32 = 6040;
pub const LCU_CURRENCY_CASSETTE: u32 = 6059;

/// Specifies, if cash unit can dispense, deposit cash or both.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CuKind {
    #[default]
    NotAvailable = LCU_NA,
    Deposit = LCU_DEPOSIT,
    Dispense = LCU_DISPENSE,
    Recycle = LCU_RECYCLE,
}

impl CuKind {
    /// Creates a new [CuType].
    pub const fn new() -> Self {
        Self::NotAvailable
    }

    /// Creates a new [CuType] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            LCU_NA => Self::NotAvailable,
            LCU_DEPOSIT => Self::Deposit,
            LCU_DISPENSE => Self::Dispense,
            LCU_RECYCLE => Self::Recycle,
            _ => Self::NotAvailable,
        }
    }
}

impl From<&CuKind> for &'static str {
    fn from(val: &CuKind) -> Self {
        match val {
            CuKind::NotAvailable => "N/A",
            CuKind::Deposit => "deposit",
            CuKind::Dispense => "dispense",
            CuKind::Recycle => "recycle",
        }
    }
}

impl From<CuKind> for &'static str {
    fn from(val: CuKind) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CuKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(CuKind, "cuKind");

/// Type of cash unit.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CuType {
    #[default]
    NotAvailable = LCU_NA,
    BillCassette = LCU_BILL_CASSETTE,
    RejectCassette = LCU_REJECT_CASSETTE,
}

impl CuType {
    /// Creates a new [CuType].
    pub const fn new() -> Self {
        Self::NotAvailable
    }

    /// Creates a new [CuType] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            LCU_NA => Self::NotAvailable,
            LCU_BILL_CASSETTE => Self::BillCassette,
            LCU_REJECT_CASSETTE => Self::RejectCassette,
            _ => Self::NotAvailable,
        }
    }
}

impl From<&CuType> for &'static str {
    fn from(val: &CuType) -> Self {
        match val {
            CuType::NotAvailable => "N/A",
            CuType::RejectCassette => "reject cassette",
            CuType::BillCassette => "bill cassette",
        }
    }
}

impl From<CuType> for &'static str {
    fn from(val: CuType) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(CuType, "cuType");

/// Represents the logical cash unit in the CDR.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum LCU {
    Dispense = LCU_DISPENSE,
    Deposit = LCU_DEPOSIT,
    Recycle = LCU_RECYCLE,
    NA = LCU_NA,
    BaitTrap = LCU_BAIT_TRAP,
    RejectCassette = LCU_REJECT_CASSETTE,
    OverflowCassette = LCU_OVERFLOW_CASSETTE,
    BillCassette = LCU_BILL_CASSETTE,
    CoinCylinder = LCU_COIN_CYLINDER,
    CoinDispenser = LCU_COIN_DISPENSER,
    RetractCassette = LCU_RETRACT_CASSETTE,
    Coupon = LCU_COUPON,
    Document = LCU_DOCUMENT,
    Escrow = LCU_ESCROW,
    #[default]
    Unknown = LCU_UNKNOWN,
    Ok = LCU_OK,
    Full = LCU_FULL,
    High = LCU_HIGH,
    Low = LCU_LOW,
    Empty = LCU_EMPTY,
    Inoperable = LCU_INOP,
    Missing = LCU_MISSING,
    NoValue = LCU_NO_VALUE,
    NoRef = LCU_NO_REF,
    NotDispensable = LCU_NOT_DISPENSABLE,
    CurrencyCassette = LCU_CURRENCY_CASSETTE,
}

impl LCU {
    /// Creates a new [LCU].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [LCU] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            LCU_DISPENSE => Self::Dispense,
            LCU_DEPOSIT => Self::Deposit,
            LCU_RECYCLE => Self::Recycle,
            LCU_NA => Self::NA,
            LCU_BAIT_TRAP => Self::BaitTrap,
            LCU_REJECT_CASSETTE => Self::RejectCassette,
            LCU_OVERFLOW_CASSETTE => Self::OverflowCassette,
            LCU_BILL_CASSETTE => Self::BillCassette,
            LCU_COIN_CYLINDER => Self::CoinCylinder,
            LCU_COIN_DISPENSER => Self::CoinDispenser,
            LCU_RETRACT_CASSETTE => Self::RetractCassette,
            LCU_COUPON => Self::Coupon,
            LCU_DOCUMENT => Self::Document,
            LCU_ESCROW => Self::Escrow,
            LCU_UNKNOWN => Self::Unknown,
            LCU_OK => Self::Ok,
            LCU_FULL => Self::Full,
            LCU_HIGH => Self::High,
            LCU_LOW => Self::Low,
            LCU_EMPTY => Self::Empty,
            LCU_INOP => Self::Inoperable,
            LCU_MISSING => Self::Missing,
            LCU_NO_VALUE => Self::NoValue,
            LCU_NO_REF => Self::NoRef,
            LCU_NOT_DISPENSABLE => Self::NotDispensable,
            LCU_CURRENCY_CASSETTE => Self::CurrencyCassette,
            _ => Self::Unknown,
        }
    }
}

impl From<LCU> for &'static str {
    fn from(val: LCU) -> Self {
        match val {
            LCU::Dispense => "dispense",
            LCU::Deposit => "deposit",
            LCU::Recycle => "recycle",
            LCU::BaitTrap => "bait trap",
            LCU::NA => "N/A",
            LCU::RejectCassette => "reject cassette",
            LCU::OverflowCassette => "overflow cassette",
            LCU::BillCassette => "bill cassette",
            LCU::CoinCylinder => "coin cylinder",
            LCU::CoinDispenser => "coin dispenser",
            LCU::RetractCassette => "retract cassette",
            LCU::Coupon => "coupon",
            LCU::CurrencyCassette => "currency cassette",
            LCU::Document => "document",
            LCU::Escrow => "escrow",
            LCU::Unknown => "unknown",
            LCU::Ok => "OK",
            LCU::Full => "full",
            LCU::High => "high",
            LCU::Low => "low",
            LCU::Empty => "empty",
            LCU::Inoperable => "inoperable",
            LCU::Missing => "missing",
            LCU::NoValue => "no value",
            LCU::NoRef => "no ref",
            LCU::NotDispensable => "not dispensable",
        }
    }
}

impl From<&LCU> for &'static str {
    fn from(val: &LCU) -> Self {
        (*val).into()
    }
}

impl fmt::Display for LCU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl_xfs_enum!(LCU, "lcu");
