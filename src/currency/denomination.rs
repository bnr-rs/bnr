use std::fmt;

pub const DENOM_ITEM_LEN: usize = 20;

/// Represents the logical cash unit in the CDR.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LogicalCashUnit {
    Dispense = bnr_sys::XFS_C_CDR_LCU_DISPENSE,
    Deposit = bnr_sys::XFS_C_CDR_LCU_DEPOSIT,
    Recycle = bnr_sys::XFS_C_CDR_LCU_RECYCLE,
    BaitTrap = bnr_sys::XFS_C_CDR_LCU_BAIT_TRAP,
    NA = bnr_sys::XFS_C_CDR_LCU_NA,
    RejectCassette = bnr_sys::XFS_C_CDR_LCU_REJECT_CASSETTE,
    OverflowCassette = bnr_sys::XFS_C_CDR_LCU_OVERFLOW_CASSETTE,
    BillCassette = bnr_sys::XFS_C_CDR_LCU_BILL_CASSETTE,
    CoinCylinder = bnr_sys::XFS_C_CDR_LCU_COIN_CYLINDER,
    CoinDispenser = bnr_sys::XFS_C_CDR_LCU_COIN_DISPENSER,
    RetractCassette = bnr_sys::XFS_C_CDR_LCU_RETRACT_CASSETTE,
    Coupon = bnr_sys::XFS_C_CDR_LCU_COUPON,
    CurrencyCassette = bnr_sys::XFS_C_CDR_LCU_CURRENCY_CASSETTE,
    Document = bnr_sys::XFS_C_CDR_LCU_DOCUMENT,
    Escrow = bnr_sys::XFS_C_CDR_LCU_ESCROW,
    #[default]
    Unknown = bnr_sys::XFS_C_CDR_LCU_UNKNOWN,
    Ok = bnr_sys::XFS_C_CDR_LCU_OK,
    Full = bnr_sys::XFS_C_CDR_LCU_FULL,
    High = bnr_sys::XFS_C_CDR_LCU_HIGH,
    Low = bnr_sys::XFS_C_CDR_LCU_LOW,
    Empty = bnr_sys::XFS_C_CDR_LCU_EMPTY,
    Inoperable = bnr_sys::XFS_C_CDR_LCU_INOP,
    Missing = bnr_sys::XFS_C_CDR_LCU_MISSING,
    NoValue = bnr_sys::XFS_C_CDR_LCU_NO_VALUE,
    NoRef = bnr_sys::XFS_C_CDR_LCU_NO_REF,
    NotDispensable = bnr_sys::XFS_C_CDR_LCU_NOT_DISPENSEABLE,
}

impl LogicalCashUnit {
    /// Creates a new [LogicalCashUnit].
    pub const fn new() -> Self {
        Self::Unknown
    }
}

impl From<u32> for LogicalCashUnit {
    fn from(val: u32) -> Self {
        match val {
            v if v == bnr_sys::XFS_C_CDR_LCU_DISPENSE => Self::Dispense,
            v if v == bnr_sys::XFS_C_CDR_LCU_DEPOSIT => Self::Deposit,
            v if v == bnr_sys::XFS_C_CDR_LCU_RECYCLE => Self::Recycle,
            v if v == bnr_sys::XFS_C_CDR_LCU_BAIT_TRAP => Self::BaitTrap,
            v if v == bnr_sys::XFS_C_CDR_LCU_NA => Self::NA,
            v if v == bnr_sys::XFS_C_CDR_LCU_REJECT_CASSETTE => Self::RejectCassette,
            v if v == bnr_sys::XFS_C_CDR_LCU_OVERFLOW_CASSETTE => Self::OverflowCassette,
            v if v == bnr_sys::XFS_C_CDR_LCU_BILL_CASSETTE => Self::BillCassette,
            v if v == bnr_sys::XFS_C_CDR_LCU_COIN_CYLINDER => Self::CoinCylinder,
            v if v == bnr_sys::XFS_C_CDR_LCU_COIN_DISPENSER => Self::CoinDispenser,
            v if v == bnr_sys::XFS_C_CDR_LCU_RETRACT_CASSETTE => Self::RetractCassette,
            v if v == bnr_sys::XFS_C_CDR_LCU_COUPON => Self::Coupon,
            v if v == bnr_sys::XFS_C_CDR_LCU_CURRENCY_CASSETTE => Self::CurrencyCassette,
            v if v == bnr_sys::XFS_C_CDR_LCU_DOCUMENT => Self::Document,
            v if v == bnr_sys::XFS_C_CDR_LCU_ESCROW => Self::Escrow,
            v if v == bnr_sys::XFS_C_CDR_LCU_UNKNOWN => Self::Unknown,
            v if v == bnr_sys::XFS_C_CDR_LCU_OK => Self::Ok,
            v if v == bnr_sys::XFS_C_CDR_LCU_FULL => Self::Full,
            v if v == bnr_sys::XFS_C_CDR_LCU_HIGH => Self::High,
            v if v == bnr_sys::XFS_C_CDR_LCU_LOW => Self::Low,
            v if v == bnr_sys::XFS_C_CDR_LCU_EMPTY => Self::Empty,
            v if v == bnr_sys::XFS_C_CDR_LCU_INOP => Self::Inoperable,
            v if v == bnr_sys::XFS_C_CDR_LCU_MISSING => Self::Missing,
            v if v == bnr_sys::XFS_C_CDR_LCU_NO_VALUE => Self::NoValue,
            v if v == bnr_sys::XFS_C_CDR_LCU_NO_REF => Self::NoRef,
            v if v == bnr_sys::XFS_C_CDR_LCU_NOT_DISPENSEABLE => Self::NotDispensable,
            _ => Self::Unknown,
        }
    }
}

impl From<LogicalCashUnit> for u32 {
    fn from(val: LogicalCashUnit) -> Self {
        val as u32
    }
}

impl From<LogicalCashUnit> for &'static str {
    fn from(val: LogicalCashUnit) -> Self {
        match val {
            LogicalCashUnit::Dispense => "dispense",
            LogicalCashUnit::Deposit => "deposit",
            LogicalCashUnit::Recycle => "recycle",
            LogicalCashUnit::BaitTrap => "bait trap",
            LogicalCashUnit::NA => "N/A",
            LogicalCashUnit::RejectCassette => "reject cassette",
            LogicalCashUnit::OverflowCassette => "overflow cassette",
            LogicalCashUnit::BillCassette => "bill cassette",
            LogicalCashUnit::CoinCylinder => "coin cylinder",
            LogicalCashUnit::CoinDispenser => "coin dispenser",
            LogicalCashUnit::RetractCassette => "retract cassette",
            LogicalCashUnit::Coupon => "coupon",
            LogicalCashUnit::CurrencyCassette => "currency cassette",
            LogicalCashUnit::Document => "document",
            LogicalCashUnit::Escrow => "escrow",
            LogicalCashUnit::Unknown => "unknown",
            LogicalCashUnit::Ok => "OK",
            LogicalCashUnit::Full => "full",
            LogicalCashUnit::High => "high",
            LogicalCashUnit::Low => "low",
            LogicalCashUnit::Empty => "empty",
            LogicalCashUnit::Inoperable => "inoperable",
            LogicalCashUnit::Missing => "missing",
            LogicalCashUnit::NoValue => "no value",
            LogicalCashUnit::NoRef => "no ref",
            LogicalCashUnit::NotDispensable => "not dispensable",
        }
    }
}

impl From<&LogicalCashUnit> for &'static str {
    fn from(val: &LogicalCashUnit) -> Self {
        (*val).into()
    }
}

impl fmt::Display for LogicalCashUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

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

    ///  Gets the size of the [Denomination].
    pub const fn size(&self) -> u32 {
        self.size
    }

    ///  Gets the amount of the [Denomination].
    pub const fn amount(&self) -> u32 {
        self.amount
    }

    ///  Gets the cashbox of the [Denomination].
    pub const fn cashbox(&self) -> u32 {
        self.cashbox
    }

    ///  Gets the list of [DenominationItem]s of the [Denomination].
    pub fn items(&self) -> &[DenominationItem] {
        self.items.as_ref()
    }
}

impl From<&Denomination> for bnr_sys::XfsDenomination {
    fn from(val: &Denomination) -> Self {
        Self {
            size: val.size,
            amount: val.amount,
            cashbox: val.cashbox,
            items: val.items.map(bnr_sys::XfsDenominationItem::from),
        }
    }
}

impl From<Denomination> for bnr_sys::XfsDenomination {
    fn from(val: Denomination) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsDenomination> for Denomination {
    fn from(val: &bnr_sys::XfsDenomination) -> Self {
        Self {
            size: val.size,
            amount: val.amount,
            cashbox: val.cashbox,
            items: val.items.map(DenominationItem::from),
        }
    }
}

impl From<bnr_sys::XfsDenomination> for Denomination {
    fn from(val: bnr_sys::XfsDenomination) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""denomination": {{"size": {}, "amount": {}, "cashbox": {}, "items": ["#,
            self.size, self.amount, self.cashbox
        )?;

        let items_len = self.items.len();
        for (i, item) in self.items.iter().enumerate() {
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
    unit: LogicalCashUnit,
    /// Bill count
    count: u32,
}

impl DenominationItem {
    /// Creates a new [DenominationItem].
    pub const fn new() -> Self {
        Self {
            unit: LogicalCashUnit::new(),
            count: 0,
        }
    }
}

impl From<&DenominationItem> for bnr_sys::XfsDenominationItem {
    fn from(val: &DenominationItem) -> Self {
        Self {
            unit: val.unit.into(),
            count: val.count,
        }
    }
}

impl From<DenominationItem> for bnr_sys::XfsDenominationItem {
    fn from(val: DenominationItem) -> Self {
        (&val).into()
    }
}

impl From<&bnr_sys::XfsDenominationItem> for DenominationItem {
    fn from(val: &bnr_sys::XfsDenominationItem) -> Self {
        Self {
            unit: val.unit.into(),
            count: val.count,
        }
    }
}

impl From<bnr_sys::XfsDenominationItem> for DenominationItem {
    fn from(val: bnr_sys::XfsDenominationItem) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DenominationItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""denomination_item": {{"unit": "{}", "count": {}}}"#,
            self.unit, self.count
        )
    }
}
