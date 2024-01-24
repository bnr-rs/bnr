use std::fmt;

use crate::impl_xfs_enum;

pub const DENOM_ITEM_LEN: usize = 20;

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
            v if v == LCU_NA => Self::NotAvailable,
            v if v == LCU_DEPOSIT => Self::Deposit,
            v if v == LCU_DISPENSE => Self::Dispense,
            v if v == LCU_RECYCLE => Self::Recycle,
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
            v if v == LCU_NA => Self::NotAvailable,
            v if v == LCU_BILL_CASSETTE => Self::BillCassette,
            v if v == LCU_REJECT_CASSETTE => Self::RejectCassette,
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
            v if v == LCU_DISPENSE => Self::Dispense,
            v if v == LCU_DEPOSIT => Self::Deposit,
            v if v == LCU_RECYCLE => Self::Recycle,
            v if v == LCU_NA => Self::NA,
            v if v == LCU_BAIT_TRAP => Self::BaitTrap,
            v if v == LCU_REJECT_CASSETTE => Self::RejectCassette,
            v if v == LCU_OVERFLOW_CASSETTE => Self::OverflowCassette,
            v if v == LCU_BILL_CASSETTE => Self::BillCassette,
            v if v == LCU_COIN_CYLINDER => Self::CoinCylinder,
            v if v == LCU_COIN_DISPENSER => Self::CoinDispenser,
            v if v == LCU_RETRACT_CASSETTE => Self::RetractCassette,
            v if v == LCU_COUPON => Self::Coupon,
            v if v == LCU_DOCUMENT => Self::Document,
            v if v == LCU_ESCROW => Self::Escrow,
            v if v == LCU_UNKNOWN => Self::Unknown,
            v if v == LCU_OK => Self::Ok,
            v if v == LCU_FULL => Self::Full,
            v if v == LCU_HIGH => Self::High,
            v if v == LCU_LOW => Self::Low,
            v if v == LCU_EMPTY => Self::Empty,
            v if v == LCU_INOP => Self::Inoperable,
            v if v == LCU_MISSING => Self::Missing,
            v if v == LCU_NO_VALUE => Self::NoValue,
            v if v == LCU_NO_REF => Self::NoRef,
            v if v == LCU_NOT_DISPENSABLE => Self::NotDispensable,
            v if v == LCU_CURRENCY_CASSETTE => Self::CurrencyCassette,
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

/// This structure handles a list of [DenominationItem]s.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Denomination {
    /// Size of items array.
    size: u32,
    /// Amount in MDU
    amount: u32,
    /// Amount the BNR cannot denominate or dispense
    cashbox: u32,
    /// The [DenominationItem]s
    items: [DenominationItem; DENOM_ITEM_LEN],
}

impl Denomination {
    /// Creates a new [Denomination].
    pub const fn new() -> Self {
        Self {
            size: 0,
            amount: 0,
            cashbox: 0,
            items: [DenominationItem::new(); DENOM_ITEM_LEN],
        }
    }

    /// Gets the size of the [Denomination].
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Sets the size of the [Denomination].
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Builder function that sets the size of the [Denomination].
    pub fn with_size(mut self, size: u32) -> Self {
        self.set_size(size);
        self
    }

    /// Gets the amount of the [Denomination].
    pub const fn amount(&self) -> u32 {
        self.amount
    }

    /// Sets the amount of the [Denomination].
    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
    }

    /// Builder function that sets the amount of the [Denomination].
    pub fn with_amount(mut self, amount: u32) -> Self {
        self.set_amount(amount);
        self
    }

    /// Gets the cashbox of the [Denomination].
    pub const fn cashbox(&self) -> u32 {
        self.cashbox
    }

    /// Sets the cashbox of the [Denomination].
    pub fn set_cashbox(&mut self, cashbox: u32) {
        self.cashbox = cashbox;
    }

    /// Builder function that sets the cashbox of the [Denomination].
    pub fn with_cashbox(mut self, cashbox: u32) -> Self {
        self.set_cashbox(cashbox);
        self
    }

    /// Gets the list of [DenominationItem]s of the [Denomination].
    pub fn items(&self) -> &[DenominationItem] {
        self.items.as_ref()
    }

    /// Gets the mutable list of [DenominationItem]s of the [Denomination].
    pub fn items_mut(&mut self) -> &mut [DenominationItem] {
        self.items.as_mut()
    }

    /// Sets the list of [DenominationItem]s of the [Denomination].
    pub fn set_items(&mut self, items: [DenominationItem; DENOM_ITEM_LEN]) {
        self.items = items;
    }

    /// Builder function that sets the list of [DenominationItem]s of the [Denomination].
    pub fn with_items(mut self, items: [DenominationItem; DENOM_ITEM_LEN]) -> Self {
        self.set_items(items);
        self
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"size": {}, "amount": {}, "cashbox": {}, "items": ["#,
            self.size, self.amount, self.cashbox
        )?;

        let items = self.items();
        let items_len = items.len();
        for (i, item) in items.iter().enumerate() {
            write!(f, "{item}")?;

            if i != items_len - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]}}")
    }
}

/// This structure describes the number of bills stored to or dispensed from a Logical Cash Unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DenominationItem {
    /// Logical Cash Unit number
    unit: u32,
    /// Bill count
    count: u32,
}

impl DenominationItem {
    /// Creates a new [DenominationItem].
    pub const fn new() -> Self {
        Self { unit: 0, count: 0 }
    }

    /// Gets the [LogicalCashUnit] number.
    pub const fn unit(&self) -> u32 {
        self.unit
    }

    /// Sets the [LogicalCashUnit] number.
    pub fn set_unit(&mut self, unit: u32) {
        self.unit = unit;
    }

    /// Builder function that sets the [LogicalCashUnit] number.
    pub fn with_unit(mut self, unit: u32) -> Self {
        self.set_unit(unit);
        self
    }

    /// Gets the count of [DenominationItem] notes.
    pub const fn count(&self) -> u32 {
        self.count
    }

    /// Sets the count of [DenominationItem] notes.
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    /// Builder function that sets the count of [DenominationItem] notes.
    pub fn with_count(mut self, count: u32) -> Self {
        self.set_count(count);
        self
    }
}

impl fmt::Display for DenominationItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"unit":{},"count":{}}}"#, self.unit, self.count)
    }
}
